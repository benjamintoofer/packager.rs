use crate::container::isobmff::nal::{nal_unit::NALUnit, NALType};
use crate::codec::h264::sequence_parameter_set::SequenceParameterSet;
use crate::container::transport_stream::{
    pes_packet, program_association_table::ProgramAssociationTable,
    program_map_table::ProgramMapTable, ts_packet,
};
use crate::error::CustomError;

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
    };
    let packets: Vec<ts_packet::TransportPacket> = Vec::new();
    let mut accumulated_pes_payload: Vec<u8> = Vec::new();
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
        let sps = SequenceParameterSet::parse(sps).unwrap();
        // sps
        println!("PPS DATA: {:?}", pps);
    });
    avc_extractor.listen_for_media_data(|media| {});

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
        // if pes.packet_start_code_prefix == 1 {
        //   total_video_frames = total_video_frames + 1;
        //   println!("NEW AUD @ {} :: {} :: stream_id {}", counter, total_video_frames, pes.stream_id);
        //   // println!("PAYLOAD: {:02X?}", pes.payload_data);

        // }
        avc_extractor.accumulate_pes_payload(pes.payload_data);
        // accumulated_pes_payload.append(&mut pes.payload_data.to_vec());
      }

      // Audio PES
      if packet.pid == audio_elem_pid {
          println!("AUDIO PID");
      }

      index = index + TS_PACKET_SIZE;
    }
    println!("TOTAL VIDEO PACKETS {}", counter);
    Ok((vec![], vec![]))
}

pub fn remux_ts_to_mp4_media_only(ts_file: &[u8]) -> Result<Vec<u8>, CustomError> {
    // TODO
    Ok(vec![])
}

struct AVCExtracter<IF, MF>
where
    IF: Fn(&Vec<u8>, &Vec<u8>),
    MF: Fn(&Vec<u8>),
{
    sps_nal: Vec<u8>,
    pps_nal: Vec<u8>,
    media_nal: Vec<u8>,
    bucket: Vec<u8>,
    init_callback: Option<IF>,
    media_callback: Option<MF>,
}

impl<IF, MF> AVCExtracter<IF, MF>
where
    IF: Fn(&Vec<u8>, &Vec<u8>),
    MF: Fn(&Vec<u8>),
{
    fn accumulate_pes_payload(&mut self, pes_payload: &[u8]) {
      let mut index: usize = 0;
      let mut nal_start_index = index;
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
                  cb(&self.sps_nal, &self.pps_nal);
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
                self.media_nal.append(&mut nal_unit.to_vec());
            }
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
