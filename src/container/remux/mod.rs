use crate::container::isobmff::nal::{nal_unit::NALUnit, NALType};
use crate::container::transport_stream::{
    pes_packet, program_association_table::ProgramAssociationTable,
    program_map_table::ProgramMapTable, ts_packet,
};
use crate::container::writer::mp4_writer::Mp4Writer;
use crate::error::CustomError;

use std::{fs::File, io::Write};

static SYNC_BYTE: u8 = 0x47;
static TS_PACKET_SIZE: usize = 188;

pub fn remux_ts_to_mp4(ts_file: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CustomError> {
    let mut avc_extractor = AVCExtracter {
        sps_nal: vec![],
        pps_nal: vec![],
        media_nal: vec![],
        bucket: vec![],
        init_callback: None,
        media_callback: None,
        all_same_timestamps: true,
        signed_comp_offset: false,
    };
    let packets: Vec<ts_packet::TransportPacket> = Vec::new();
    let mut accumulated_pes_payload: Vec<u8> = Vec::new();
    let minimum_decode_time: u64 = u64::MAX;
    let mut index = 0usize;

    let mut pat: ProgramAssociationTable;
    let mut pmt: ProgramMapTable;

    let mut program_map_pid: u16 = u16::max_value();
    let mut video_elem_pid = u16::max_value();
    let mut audio_elem_pid = u16::max_value();

    let mut counter = 0;
    let mut total_video_frames = 0;

    avc_extractor.listen_for_init_data(|sps, pps| {
        println!("SPS DATA: {:02X?}", sps);
        println!("PPS DATA: {:?}", pps);
        let init_segment = Mp4Writer::create_mp4_writer()
          .timescale(90000)
          .sps(sps)
          .pps(pps)
          .build_init_segment();

        match init_segment {
            Ok(x) => {
              let mut file = File::create("/Users/benjamintoofer/Desktop/my_own_init.mp4").unwrap();
              match file.write_all(&x) {
                  Ok(_) => {println!("FINISHED WRITING SEGMENT!!!")}
                  Err(_) => {println!("FUCKED UP WRITING SEGMENT")}
              }
            }
            Err(err) => {
              println!("{:?}", err);
            }
        }
    });
    avc_extractor.listen_for_media_data(|media| {
      println!("LISTEN MEDIA DATA");
      println!("COUNT {}", media.len());
    });

    while index < ts_file.len() {
      if ts_file[index] != SYNC_BYTE {
        // Out of sync so find the next sync point
        index = index + 1;
        continue;
      }
      let packet = ts_packet::TransportPacket::parse(ts_file[index..(index + TS_PACKET_SIZE)].as_ref())?;
      // ProgramAssociationTable
      if packet.pid == 0 {
        pat = ProgramAssociationTable::parse(packet.data, packet.payload_unit_start_indicator)
          .unwrap();
        program_map_pid = pat.program_map_pid;
      }

      // ProgramMapTable
      if packet.pid == program_map_pid {
        pmt = ProgramMapTable::parse(packet.data, packet.payload_unit_start_indicator).unwrap();
        // Video
        if let Some(stream_info) = pmt.video_stream_info {
          video_elem_pid = stream_info.pid;
        }
        // Audio
        if let Some(stream_info) = pmt.audio_stream_info {
          audio_elem_pid = stream_info.pid;
        }
      }

      // Video PES
      if packet.pid == video_elem_pid {
        counter = counter + 1;
        let pes = pes_packet::PESPacket::parse(packet.data)?;
        if let Some(pts) = pes.pts {
          println!("PTS {}; DTS: {}", pts, pes.dts.unwrap());
        }
        avc_extractor.accumulate_pes_payload(pes);
      }

      // Audio PES
      if packet.pid == audio_elem_pid {
          println!("AUDIO PID");
      }

      index = index + TS_PACKET_SIZE;
    }
    avc_extractor.flush_final_media();
    println!("TOTAL VIDEO PACKETS {}", counter);
    Ok((vec![], vec![]))
}

pub fn remux_ts_to_mp4_media_only(ts_file: &[u8]) -> Result<Vec<u8>, CustomError> {
    // TODO
    Ok(vec![])
}

struct NalRep {
  nal_unit: Vec<u8>,
  dts: u64,
  pts: u64,
}
struct AVCExtracter<IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<Vec<u8>>),
{
  sps_nal: Vec<u8>,
  pps_nal: Vec<u8>,
  media_nal: Vec<Vec<u8>>,
  bucket: Vec<u8>,
  init_callback: Option<IF>,
  media_callback: Option<MF>,
  signed_comp_offset: bool,
  all_same_timestamps: bool,
}

impl<IF, MF> AVCExtracter<IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<Vec<u8>>),
{
  fn is_all_same_timestamps(self) -> bool {
    self.all_same_timestamps
  }

  fn is_signed_comp_offset(self) -> bool {
    self.signed_comp_offset
  }

  fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) {
    let mut index: usize = 0;
    let mut nal_start_index = index;
    let pes_payload = pes.payload_data;
    loop {
      if (index + 4) >= pes_payload.len() { // If the next 4 bytes is greater than the total payload, add it to the bucket for the next pes payload
          let mut leftover: Vec<u8> = pes_payload[nal_start_index..pes_payload.len()].to_vec();
          self.bucket.append(&mut leftover);
          break;
      }
      let boundary = NALUnit::find_boundary(index, pes_payload);
      if boundary == -1 { // If no nal boundary is found, increment the index and continue searching for the next boundary
          index += 1;
          continue;
      }
      let mut nal_unit: Vec<u8> = vec![];
      if !self.bucket.is_empty() {
        nal_unit = self.bucket.clone();
        self.bucket.clear();
      }

      if nal_start_index != index {
        let mut part_payload: Vec<u8> = pes_payload[nal_start_index..index].to_vec();
        nal_unit.append(&mut part_payload);
      }

      if !nal_unit.is_empty() {
        let nal_unit_value = nal_unit[0] & 0x1F;
        let nal_type = match NALType::get_type(nal_unit_value) {
          Ok(nal_type) => nal_type,
          Err(err) => {
            println!("ERROR!!");
            println!("{}", err.to_string());
            continue;
          }
        };
        println!("NAL TYPE {:?}", nal_type.value());
        self.handle_nal_unit(nal_type, &nal_unit);
        // Have the data to create the init segment
        if self.sps_nal.len() > 0 && self.pps_nal.len() > 0 {
            if let Some(cb) = &self.init_callback {
                let sps = self.sps_nal[0..].to_vec();
                let pps = self.pps_nal[0..].to_vec();
                self.sps_nal.clear();
                self.pps_nal.clear();
                cb(&sps, &pps);
            }
        }
      }
      index += boundary as usize;
      nal_start_index = index;
    }
  }

  fn listen_for_init_data(&mut self, callback: IF) -> &Self {
      self.init_callback = Some(callback);
      return self;
  }

  fn listen_for_media_data(&mut self, callback: MF) -> &Self {
      self.media_callback = Some(callback);
      return self;
  }

  fn handle_nal_unit(&mut self, nal_type: NALType, nal_unit: &[u8]) {
    match nal_type {
      NALType::SPS => {
        self.sps_nal = nal_unit.to_vec();
      }
      NALType::PPS => self.pps_nal = nal_unit.to_vec(),
      NALType::AUD => {}
      _ => {
        println!("NAL MEDIA SIZE: {}", nal_unit.len());
        // self.media_nal.append(&mut nal_unit.to_vec());
        self.media_nal.push(nal_unit.to_vec())
      }
    }
  }

  fn flush_final_media(&mut self) {
    self.media_nal.push(self.bucket.to_vec());
    if let Some(cb) = &self.media_callback {
      cb(&self.media_nal);
    }
  }
}

use crate::error::{construct_error, error_code::{RemuxMinorCode, MajorCode}};

pub fn generate_error(message: String) -> CustomError {
  return  construct_error(
    MajorCode::REMUX, 
    Box::new(RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR),
    message,
    file!(), 
    line!());
}
