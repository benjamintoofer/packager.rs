use crate::{container::isobmff::nal::{nal_unit::NALUnit, NALType}, util::bit_reader::BitReader};
use crate::container::transport_stream::{
    pes_packet, program_association_table::ProgramAssociationTable,
    program_map_table::ProgramMapTable, ts_packet,
};
use crate::container::writer::mp4_writer::Mp4Writer;
use crate::error::CustomError;
use crate::container::isobmff::nal::NalRep;

use core::panic;
use std::{fs::File, io::Write};

pub mod extractor;

static SYNC_BYTE: u8 = 0x47;
static TS_PACKET_SIZE: usize = 188;

pub fn remux_ts_to_mp4(ts_file: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CustomError> {
    let mut avc_extractor = AVCExtractor::create();
    let mut aac_extractor = AACExtractor::create();
    let mut index = 0usize;

    let mut pat: ProgramAssociationTable;
    let mut pmt: ProgramMapTable;

    let mut program_map_pid: u16 = u16::max_value();
    let mut video_elem_pid = u16::max_value();
    let mut audio_elem_pid = u16::max_value();

    // AAC
    aac_extractor.listen_for_init_data(|adts_frame| {
      let init_segment = Mp4Writer::create_mp4_writer()
        // .timescale(timescale)
        .build_init_segment();
    });

    aac_extractor.listen_for_media_data(|adts_frame| {
      let init_segment = Mp4Writer::create_mp4_writer()
        // .timescale(timescale)
        .build_init_segment();
    });

    // AVC
    avc_extractor.listen_for_init_data(|sps, pps| {
        println!("SPS DATA: {:02X?}", sps);
        println!("PPS DATA: {:?}", pps);
        let init_segment = Mp4Writer::create_mp4_writer()
          .timescale(90000)
          .sps(sps)
          .pps(pps)
          .build_init_segment(
             AVCSampleEntryBuilder::create_builder()
                                  .sample_entry(
                                    SampleEntryBuilder::create_builder()
                                  )
                                  .visual_sample_entry(
                                    VisualSampleEntryBuilder::create_builder()
                                      .sps(&self.sps)
                                  )
                                  .avc_c(
                                    AVCDecoderConfigurationRecordBuilder::create_builder()
                                      .sps(&self.sps)
                                      .pps(&self.pps)
                                  )
          );

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
      let media_segment = Mp4Writer::create_mp4_writer()
          .timescale(90000)
          .nals(media.clone())
          .build_media_segment();

      match media_segment {
            Ok(x) => {
              let mut file = File::create("/Users/benjamintoofer/Desktop/my_own_media.mp4").unwrap();
              match file.write_all(&x) {
                  Ok(_) => {println!("FINISHED WRITING MEDIA SEGMENT!!!")}
                  Err(_) => {println!("FUCKED UP WRITING MEDIA SEGMENT")}
              }
            }
            Err(err) => {
              println!("{:?}", err);
            }
        }
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
        // let pes = pes_packet::PESPacket::parse(packet.data)?;
        // avc_extractor.accumulate_pes_payload(pes)?;
      }

      // Audio PES
      if packet.pid == audio_elem_pid {
        match pmt.audio_stream_info.unwrap().stream_type {

        }
        let pes = pes_packet::PESPacket::parse(packet.data)?;
        println!("AUDIO PTS: {:?}; DTS: {:?}", pes.pts, pes.dts);
        println!("PES DATA: {:02X?}", pes.payload_data);
        panic!("DONE");
        // aac_extractor.accumulate_pes_payload(pes)?;
      }

      index = index + TS_PACKET_SIZE;
    }
    avc_extractor.flush_final_media();
    aac_extractor.flush_final_media()?;
    Ok((vec![], vec![]))
}

pub fn remux_ts_to_mp4_media_only(ts_file: &[u8]) -> Result<Vec<u8>, CustomError> {
    // TODO
    Ok(vec![])
}

struct AACExtractor <IF, MF>
where
  IF: Fn(&ADTSFrame),
  MF: Fn(&Vec<ADTSFrame>),
{
  bucket: Vec<u8>,
  current_pts: u64,
  current_dts: u64,
  adts_frames: Vec<ADTSFrame>,
  init_callback: Option<IF>,
  media_callback: Option<MF>,
}

impl<IF, MF> AACExtractor<IF, MF>
where
  IF: Fn(&ADTSFrame),
  MF: Fn(&Vec<ADTSFrame>),
{
  pub fn create() -> AACExtractor<IF, MF> {
    AACExtractor {
      bucket: vec![],
      adts_frames: vec![],
      current_pts: 0,
      current_dts: 0,
      init_callback: None,
      media_callback: None,
    }
  }

  pub fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) -> Result<(), CustomError> {

    // Flush bucket since we are encountering a new ADTS sequence
    if pes.pts.is_some() && !self.bucket.is_empty() {
      let adts_packet = self.bucket.clone();
      self.bucket.clear();

     let mut adts_frames = ADTS::parse(&adts_packet)?;
     self.adts_frames.append(&mut adts_frames);

     // If we have am aac frame, we can immediatley begin generating the init segment

    }

    if let Some(pts) = pes.pts {
      let dts = pes.dts.map_or_else(||pts, |dts|dts);
      self.current_dts = dts;
      self.current_pts = pts;
    }

    self.bucket.append(&mut pes.payload_data.to_vec());

    Ok(())
  }

  fn flush_final_media(&mut self) -> Result<(), CustomError> {
    let mut adts_frames = ADTS::parse(&self.bucket)?;
    self.adts_frames.append(&mut adts_frames);
    Ok(())
  }

  fn listen_for_init_data(&mut self, callback: IF) -> &Self {
      self.init_callback = Some(callback);
      return self;
  }

  fn listen_for_media_data(&mut self, callback: MF) -> &Self {
      self.media_callback = Some(callback);
      return self;
  }
}


struct AVCExtractor<IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<NalRep>),
{
  sps_nal: Vec<u8>,
  pps_nal: Vec<u8>,
  media_nal: Vec<NalRep>,
  bucket: Vec<u8>,
  init_callback: Option<IF>,
  media_callback: Option<MF>,
  signed_comp_offset: bool,
  all_same_timestamps: bool,
  current_pts: u64,
  current_dts: u64,
}

impl<IF, MF> AVCExtractor<IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<NalRep>),
{
  fn create() -> AVCExtractor<IF, MF> {
    AVCExtractor {
      sps_nal: vec![],
      pps_nal: vec![],
      media_nal: vec![],
      bucket: vec![],
      init_callback: None,
      media_callback: None,
      all_same_timestamps: true,
      signed_comp_offset: false,
      current_pts: 0,
      current_dts: 0,
    }
  }

  fn is_all_same_timestamps(self) -> bool {
    self.all_same_timestamps
  }

  fn is_signed_comp_offset(self) -> bool {
    self.signed_comp_offset
  }

  fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) -> Result<(), CustomError> {
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
        let nal_type = NALType::get_type(nal_unit_value)?;

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

    if let Some(pts) = pes.pts {
      // Can assume dts is there because the pes parser will set it if its not there
      let dts = pes.dts.unwrap();
      // Set the flag that the composition offset will be negative. Will set the version in trun to 1
      if dts > pts {
        self.signed_comp_offset = true;
      }

      // Determine if we'll need to set the compoisiton offset in the trun
      if pts > dts {
        self.all_same_timestamps = false;
      }

      self.current_pts = pts;
      self.current_dts = dts;
    }

    Ok(())
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
        self.media_nal.push(NalRep{
          nal_unit: nal_unit.to_vec(),
          pts: self.current_pts,
          dts: self.current_dts,
        })
      }
    }
  }

  fn flush_final_media(&mut self) {
    self.media_nal.push(NalRep{
          nal_unit: self.bucket.to_vec(),
          pts: self.current_pts,
          dts: self.current_dts,
        });
    if let Some(cb) = &self.media_callback {
      cb(&self.media_nal);
    }
  }
}

use crate::error::{construct_error, error_code::{RemuxMinorCode, MajorCode}};

use super::transport_stream::adts::{ADTS, ADTSFrame};

pub fn generate_error(message: String) -> CustomError {
  return  construct_error(
    MajorCode::REMUX, 
    Box::new(RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR),
    message,
    file!(), 
    line!());
}
