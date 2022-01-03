use std::u16;

use crate::{error::CustomError, util};


#[derive(Debug)]
pub struct ProgramAssociationTable {
  table_id: u8,
  section_syntax_indicator: bool,
  section_length: u16,                // u12
  transport_stream_id: u16,
  version_number: u8,                 // u5
  current_next_indicator: bool,
  section_number: u8,
  last_section_number: u8,
  pub program_map_pid: u16,               // u13
  crc_32: u32

}

impl ProgramAssociationTable {

  pub fn parse(data: &[u8], payload_unit_start_indicator: bool) -> Result<ProgramAssociationTable, CustomError> {
    let mut start = 0usize;
    if payload_unit_start_indicator {
      start = util::get_u8(data, start)? as usize + 1;
    }
    let table_id = util::get_u8(data, start)?;

    start = start + 1;
    let temp_16 = util::get_u16(data, start)?;
    let section_syntax_indicator = (temp_16 & 0x8000) != 0;
    let section_length = temp_16 & 0xFFF;

    start = start + 2;
    let end = start + section_length as usize;
    let transport_stream_id = util::get_u16(data, start)?;

    start = start + 2;
    let temp_8 = util::get_u8(data, start)?;
    let version_number = (temp_8 & 0x3E) >> 1;
    let current_next_indicator = (temp_8 & 1) != 0;

    start = start + 1;
    let section_number = util::get_u8(data, start)?;

    start = start + 1;
    let last_section_number = util::get_u8(data, start)?;
    start = start + 1;

    let mut program_map_pid = 0;
    // Grabbing only one program map id. Don't really care if there's more
    while start < (end - 4) {
      let program_number = util::get_u16(data, start)?;
      start = start + 2;
      if program_number == 0 {
        // Ignore network _id
      } else {
        program_map_pid = util::get_u16(data, start)? & 0x1FFF;
      }
      start = start + 2;
    }

    let crc_32 = util::get_u32(data, start)?;

    Ok(
      ProgramAssociationTable{
        table_id,
        section_syntax_indicator,
        section_length,
        transport_stream_id,
        version_number,
        current_next_indicator,
        section_number,
        last_section_number,
        program_map_pid,
        crc_32,
      }
    )
  }
}