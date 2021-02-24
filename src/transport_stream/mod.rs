use program_association_table::ProgramAssociationTable;
use program_map_table:: ProgramMapTable;

use crate::error::CustomError;

pub mod program_association_table;
pub mod program_map_table;
pub mod ts_packet;
pub mod pes_packet;
pub mod elementary_stream_type;

static SYNC_BYTE: u8 = 0x47;
static TS_PACKET_SIZE: usize = 188;

pub fn parse_transport_stream(ts_file: &[u8]) -> Result<Vec<ts_packet::TransportPacket>, CustomError> {
  let packets: Vec<ts_packet::TransportPacket> = Vec::new();
  let mut index = 0usize;
  
  let mut pat: ProgramAssociationTable;
  let mut pmt: ProgramMapTable;

  let mut program_map_pid: u16 = u16::max_value();
  let mut video_elem_pid = u16::max_value();
  let mut audio_elem_pid = u16::max_value();

  while index < ts_file.len() {

    if ts_file[index] != SYNC_BYTE {
      // Out of sync so find the next sync point
      index = index + 1;
      continue;
    }
    let packet = ts_packet::TransportPacket::parse(ts_file[index..(index + TS_PACKET_SIZE)].as_ref());
    let temp = packet.unwrap();
    // ProgramAssociationTable
    if temp.pid == 0 {
      pat = ProgramAssociationTable::parse(temp.data, temp.payload_unit_start_indicator).unwrap();
      program_map_pid =  pat.program_map_pid;
    }

    // ProgramMapTable
    if temp.pid == program_map_pid {
      pmt = ProgramMapTable::parse(temp.data, temp.payload_unit_start_indicator).unwrap();
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
    if temp.pid == video_elem_pid {
      println!("VIDEO PID");
    }

    // Audio PES
    if temp.pid == audio_elem_pid {
      println!("AUDIO PID");
    }

    index = index + TS_PACKET_SIZE;
  }
  Ok(vec![])
}
