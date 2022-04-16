use std::u16;

use crate::error::CustomError;
use crate::util;
use super::elementary_stream_type::ElementaryStreamType;

#[derive(Debug)]
pub struct StreamInfo {
  pub pid: u16,
  pub stream_type: ElementaryStreamType,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ProgramMapTable {
  table_id: u8,
  section_syntax_indicator: bool,
  section_length: u16,                    // 12 bit
  program_number: u16,                    
  version_number: u8,                     // 5 bit
  current_next_indicator: bool,
  section_number: u8,
  last_section_number: u8,
  PCR_PID: u16,                           // 13 bit
  program_info_length: u16,               // 12 bit
  pub audio_stream_info: Option<StreamInfo>,
  pub video_stream_info: Option<StreamInfo>,
}

#[allow(non_snake_case)]
impl ProgramMapTable {
  pub fn parse(data: &[u8], payload_unit_start_indicator: bool) -> Result<ProgramMapTable, CustomError> {
    let mut start = 0usize;
    if payload_unit_start_indicator {
      start = util::get_u8(data, start)? as usize + 1;
    }

    let table_id = util::get_u8(data,start)?;

    start = start + 1;
    let buffer_16 = util::get_u16(data, start)?;
    let section_syntax_indicator = (buffer_16 & 0x8000) != 0;
    let section_length = buffer_16 & 0xFFF;

    start = start + 2;
    let end = start + section_length as usize;
    let program_number = util::get_u16(data, start)?;

    start = start + 2;
    let buffer_8 = util::get_u8(data, start)?;
    let version_number = (buffer_8 & 0x3E) >> 1;
    let current_next_indicator = (buffer_8 & 0x1) != 0;

    start = start + 1;
    let section_number = util::get_u8(data, start)?;

    start = start + 1;
    let last_section_number = util::get_u8(data, start)?;

    start = start + 1;
    let PCR_PID = util::get_u16(data, start)? & 0x1FFF;

    start = start + 2;
    let program_info_length = util::get_u16(data, start)? & 0xFFF;

    start = start + 2;
    let program_end = start + program_info_length as usize;

    // NOTE (benjamintoofer@gmail.com): Skip descriptors for now
    start = program_end;

    let mut video_stream_info = None;
    let mut audio_stream_info = None;
    while start < end - 4 {
      let stream_type = util::get_u8(data, start)?;
      start = start + 1;
      let elementary_pid = util::get_u16(data, start)? & 0x1FFF;
      start = start + 2;
      let es_info_length = util::get_u16(data, start)? & 0xFFF;
      start = start + 2;
      let es_info_end = start + es_info_length as usize;
      // TODO (benjamintoofer@gmail.com): Skip es info for now
      start = es_info_end;
      let stream = ElementaryStreamType::get_type(stream_type);
      match stream {
          ElementaryStreamType::AAC => {
            audio_stream_info = Some(StreamInfo{pid: elementary_pid, stream_type: stream});
          }
          ElementaryStreamType::AC3 => {
            audio_stream_info = Some(StreamInfo{pid: elementary_pid, stream_type: stream});
          }
          ElementaryStreamType::E_AC3 => {
            audio_stream_info = Some(StreamInfo{pid: elementary_pid, stream_type: stream});
          }
          ElementaryStreamType::H_264 => {
            video_stream_info = Some(StreamInfo{pid: elementary_pid, stream_type: stream});
          }
          ElementaryStreamType::H_265 => {
            video_stream_info = Some(StreamInfo{pid: elementary_pid, stream_type: stream});
          }
          ElementaryStreamType::UNKNOWN => {}
      }
    }

    Ok(
      ProgramMapTable{
        table_id,
        section_syntax_indicator,
        section_length,
        program_number,
        version_number,
        current_next_indicator,
        section_number,
        last_section_number,
        PCR_PID,
        program_info_length,
        audio_stream_info,
        video_stream_info
      }
    )
  }
}