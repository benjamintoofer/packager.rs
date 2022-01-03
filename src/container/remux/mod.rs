use crate::error::CustomError;
use crate::container::isobmff::nal::{NALType, nal_unit::NALUnit};
use crate::container::transport_stream::{ts_packet, pes_packet, program_map_table::ProgramMapTable,program_association_table::ProgramAssociationTable};

static SYNC_BYTE: u8 = 0x47;
static TS_PACKET_SIZE: usize = 188;

pub fn remux_ts_to_mp4(ts_file: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CustomError> {
  let mut avc_extractor = AVCExtracter{
    sps_nal: vec![],
    pps_nal: vec![],
    media_nal: vec![],
    bucket: &[],
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

  avc_extractor
    .listen_for_init_data(|sps, pps|{

    });
  avc_extractor
    .listen_for_media_data(|media| {

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
      pat = ProgramAssociationTable::parse(packet.data, packet.payload_unit_start_indicator).unwrap();
      program_map_pid =  pat.program_map_pid;
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

  NALUnit::byte_stream_to_nal_units(&accumulated_pes_payload);
  println!("TOTAL VIDEO PACKETS {}", counter);
  Ok(
    (vec![], vec![])
  )
}

pub fn remux_ts_to_mp4_media_only(ts_file: &[u8]) -> Result<Vec<u8>, CustomError> {
  Ok(vec![])
}

struct AVCExtracter<'a, IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<u8>),
{
  sps_nal: Vec<u8>,
  pps_nal: Vec<u8>,
  media_nal: Vec<u8>,
  bucket: &'a[u8],
  init_callback: Option<IF>,
  media_callback: Option<MF>,
}

impl<'a, IF, MF> AVCExtracter<'a, IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<u8>),
{
  fn accumulate_pes_payload(&mut self, pes_payload: &'a [u8]) {
    let mut index: usize = 0;
    let mut nal_start_index = index;
    loop {
      if index == pes_payload.len() {
        break;
      }
      let boundary = NALUnit::find_boundary(index, pes_payload);
      // Just found a new boundary, flush the accumulated byte data
      if boundary != -1 {
        let mut nal_unit: Vec<u8> = pes_payload[nal_start_index..index].to_vec();
        if self.bucket.len() > 0 {
          let temp = [self.bucket, &nal_unit].concat();
          nal_unit = temp;
        }
        let nal_type = match NALType::get_type(nal_unit[0]) {
            Ok(nal_type) => nal_type,
            Err(err) => {
              println!("{}", err.to_string());
              continue;
            }
        };

        self.handle_nal_unit(nal_type, &nal_unit);
        // Have the data to create the init segment
        if self.sps_nal.len() > 0 && self.pps_nal.len() > 0 {
          if let Some(cb) = &self.init_callback {
            cb(&self.sps_nal, &self.pps_nal);
          }
        }

        index += boundary as usize;
        nal_start_index = index;
      }
      index += 1;
    }


    while index < pes_payload.len() {
      let offset = NALUnit::find_boundary(index, pes_payload);
      if offset == -1 {
        index += 1;
        continue;
      }
      index += offset as usize;
      let nal_start_index = index;
      println!("{:02X?} == i: {}", pes_payload[index], index);
      // Find the other nal units

      let nal_end_index = loop {
        if index == pes_payload.len() {
          break index;
        }
        if NALUnit::find_boundary(index, pes_payload) != -1 {
          break index;
        }
        index += 1;
      };
      let data = pes_payload[nal_start_index..nal_end_index].as_ref();
      println!("NAL UNIT SIZE: {}", data.len());
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
        NALType::PPS => {
          self.pps_nal = nal_unit.to_vec()
        }
        NALType::AUD => {}
        _ => {
          self.media_nal.append(&mut nal_unit.to_vec());
        }
    }
  }
}