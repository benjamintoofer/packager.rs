use std::{convert::TryInto};

use crate::error::{CustomError, construct_error};
use crate::error::error_code::{MajorCode, NalMinorCode};
use crate::util;

pub struct NALUnit {
  pub size: usize,
  rbsp_bytes: Vec<u8> 
}

impl NALUnit {
  pub fn get_ref_idc(&self) -> u8 {
    (self.rbsp_bytes[0] & 0x60) >> 5
  }

  pub fn get_unit_type(&self) -> u8 {
    self.rbsp_bytes[0] & 0x1F
  }
}

impl NALUnit {

  pub fn byte_stream_to_nal_units(byte_stream: &[u8]) -> Result<Vec<NALUnit>, CustomError> {
    let mut index: usize = 0;
    if NALUnit::find_boundary(index, byte_stream) == -1 {
      return Err(construct_error(
        MajorCode::NAL, 
        Box::new(NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX_ERROR),
        "Invalid nal unit length".to_string(), 
        file!(), 
        line!()));
    }

    while index < byte_stream.len() {
      let offset = NALUnit::find_boundary(index, byte_stream);
      if offset == -1 {
        index += 1;
        continue;
      }
      index += offset as usize;
      let nal_start_index = index;
      println!("{:02X?} == i: {} :: len: {}", byte_stream[index], index, byte_stream.len());
      // Find the other nal units

      let nal_end_index = loop {
        if index == byte_stream.len() {
          break index;
        }
        if NALUnit::find_boundary(index, byte_stream) != -1 {
          break index;
        }
        index += 1;
      };
      let data = byte_stream[nal_start_index..nal_end_index].as_ref();
      println!("NAL UNIT SIZE: {}", data.len());
    }
    return Ok(vec![])
  }

  pub fn find_boundary(index: usize, data: &[u8]) -> i8 {
    if data[index] == 0 &&
      data[index + 1]  == 0 &&
      data[index + 2]  == 0 &&
      data[index + 3]  == 1 {
        return 4;
    } else if data[index] == 0 &&
      data[index + 1]  == 0 &&
      data[index + 2]  == 1 {
        return 3;
      }

      return -1;
  }

  pub fn parse(mdat: &[u8], offset: usize, nal_unit_length: u8) -> Result<NALUnit, CustomError> {
    let nal_size = NALUnit::get_nal_unit_size(mdat, offset, nal_unit_length)?;
    let offset_without_nal_length = offset + nal_unit_length as usize;
    let nal_data = mdat[offset_without_nal_length..(offset + nal_size)].as_ref();
    let mut rbsp_bytes: Vec<u8> = vec![];
    for mut i in 0..nal_data.len() {
      if 
        i + 2 < nal_data.len() && 
        nal_data[i]     == 0x0 &&
        nal_data[i + 1] == 0x0 &&
        nal_data[i + 2] == 0x3
      {
        rbsp_bytes.push(nal_data[i]);
        rbsp_bytes.push(nal_data[i + 1]);
        i += 2;
      }
      rbsp_bytes.push(nal_data[i]);
    }

    Ok(NALUnit {
      size: nal_size,
      rbsp_bytes: rbsp_bytes
    })
  }

  fn get_nal_unit_size(mdat: &[u8], offset: usize, nal_unit_length: u8) -> Result<usize, CustomError> {
    match nal_unit_length {
      4 => {util::get_u32(mdat, offset).map(|e| e.try_into().unwrap())}
      2 => {util::get_u16(mdat, offset).map(|e| e.try_into().unwrap())}
      1 => {util::get_u8(mdat, offset).map(|e| e.try_into().unwrap())}
      _ => {Err(construct_error(
        MajorCode::NAL, 
        Box::new(NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH_ERROR),
        "Invalid nal unit length".to_string(), 
        file!(), 
        line!()))}
    }
  }

  pub fn parse_sei(rbsp_bytes: &[u8]) {
    let mut payload_type = 0u32;
    let mut offset = 0;
    while rbsp_bytes[offset] == 0xFF {
      payload_type += 0xFF;
      offset += 1;
    }
    payload_type += rbsp_bytes[offset] as u32;
    offset += 1;

    let mut payload_size = 0u32;
    while rbsp_bytes[offset] == 0xFF {
      payload_size += 0xFF;
      offset += 1;
    }
    payload_size += rbsp_bytes[offset] as u32;
    offset += 1;

    let payload = rbsp_bytes[offset..(offset + payload_size as usize)].as_ref();

    NALUnit::parse_sei_payload(payload_type, payload);
    // NOTE (benjamintoofer@gmail.com): Do something with the sei payload
  }

  fn parse_sei_payload(payload_type: u32, payload: &[u8]) -> Option<&[u8]> {
    match payload_type {
      4 => {NALUnit::user_data_registered_itu_t_t35(payload)}
      _ => {Option::None}
    }

  }

  fn user_data_registered_itu_t_t35(payload: &[u8]) -> Option<&[u8]> {
    // itu_t_t35_contry_code must be 181 (United States) for captions
    if payload[0] != 181 {
      return Option::None;
    }

    // itu_t_t35_provider_code should be 49 (ATSC) for captions
    if util::get_u16(payload, 1).unwrap_or(0) != 49 {
      return Option::None;
    }

    // the user_identifier should be "GA94" to indicate ATSC1 data
    if util::get_u32(payload, 3).unwrap_or(0) != 0x47413934 {
      return Option::None;
    }

    // user_data_type_code should be 0x03 for caption data
    if payload[7] != 0x3 {
      return Option::None;
    }

    return Option::Some(payload[8..payload.len() - 1].as_ref());
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_nal_unit_parsing() {
    
    // This is the beginning of the mdat from /assets/v_frag.mp4. I just took the mdat header with the first nal unit
    let mdat_data: [u8; 746] = [
      // size + mdat
      0x00, 0x01, 0x96, 0xDD, 0x6D, 0x64, 0x61, 0x74,
      0x00, 0x00, 0x02, 0xE2, 0x06, 0x05, 0xFF, 0xFF, 0xDE, 0xDC, 0x45, 0xE9, 0xBD, 0xE6, 0xD9, 0x48, 0xB7, 0x96, 0x2C, 0xD8,
      0x20, 0xD9, 0x23, 0xEE, 0xEF, 0x78, 0x32, 0x36, 0x34, 0x20, 0x2D, 0x20, 0x63, 0x6F, 0x72, 0x65, 0x20, 0x31, 0x34, 0x36,
      0x20, 0x72, 0x31, 0x31, 0x4D, 0x20, 0x31, 0x32, 0x31, 0x33, 0x39, 0x36, 0x63, 0x20, 0x2D, 0x20, 0x48, 0x2E, 0x32, 0x36,
      0x34, 0x2F, 0x4D, 0x50, 0x45, 0x47, 0x2D, 0x34, 0x20, 0x41, 0x56, 0x43, 0x20, 0x63, 0x6F, 0x64, 0x65, 0x63, 0x20, 0x2D,
      0x20, 0x43, 0x6F, 0x70, 0x79, 0x6C, 0x65, 0x66, 0x74, 0x20, 0x32, 0x30, 0x30, 0x33, 0x2D, 0x32, 0x30, 0x31, 0x35, 0x20,
      0x2D, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x76, 0x69, 0x64, 0x65, 0x6F, 0x6C, 0x61,
      0x6E, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x78, 0x32, 0x36, 0x34, 0x2E, 0x68, 0x74, 0x6D, 0x6C, 0x20, 0x2D, 0x20, 0x6F, 0x70,
      0x74, 0x69, 0x6F, 0x6E, 0x73, 0x3A, 0x20, 0x63, 0x61, 0x62, 0x61, 0x63, 0x3D, 0x30, 0x20, 0x72, 0x65, 0x66, 0x3D, 0x33,
      0x20, 0x64, 0x65, 0x62, 0x6C, 0x6F, 0x63, 0x6B, 0x3D, 0x31, 0x3A, 0x30, 0x3A, 0x30, 0x20, 0x61, 0x6E, 0x61, 0x6C, 0x79,
      0x73, 0x65, 0x3D, 0x30, 0x78, 0x31, 0x3A, 0x30, 0x78, 0x31, 0x31, 0x31, 0x20, 0x6D, 0x65, 0x3D, 0x68, 0x65, 0x78, 0x20,
      0x73, 0x75, 0x62, 0x6D, 0x65, 0x3D, 0x37, 0x20, 0x70, 0x73, 0x79, 0x3D, 0x31, 0x20, 0x70, 0x73, 0x79, 0x5F, 0x72, 0x64,
      0x3D, 0x31, 0x2E, 0x30, 0x30, 0x3A, 0x30, 0x2E, 0x30, 0x30, 0x20, 0x6D, 0x69, 0x78, 0x65, 0x64, 0x5F, 0x72, 0x65, 0x66,
      0x3D, 0x31, 0x20, 0x6D, 0x65, 0x5F, 0x72, 0x61, 0x6E, 0x67, 0x65, 0x3D, 0x31, 0x36, 0x20, 0x63, 0x68, 0x72, 0x6F, 0x6D,
      0x61, 0x5F, 0x6D, 0x65, 0x3D, 0x31, 0x20, 0x74, 0x72, 0x65, 0x6C, 0x6C, 0x69, 0x73, 0x3D, 0x31, 0x20, 0x38, 0x78, 0x38,
      0x64, 0x63, 0x74, 0x3D, 0x30, 0x20, 0x63, 0x71, 0x6D, 0x3D, 0x30, 0x20, 0x64, 0x65, 0x61, 0x64, 0x7A, 0x6F, 0x6E, 0x65,
      0x3D, 0x32, 0x31, 0x2C, 0x31, 0x31, 0x20, 0x66, 0x61, 0x73, 0x74, 0x5F, 0x70, 0x73, 0x6B, 0x69, 0x70, 0x3D, 0x31, 0x20,
      0x63, 0x68, 0x72, 0x6F, 0x6D, 0x61, 0x5F, 0x71, 0x70, 0x5F, 0x6F, 0x66, 0x66, 0x73, 0x65, 0x74, 0x3D, 0x2D, 0x32, 0x20,
      0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x73, 0x3D, 0x34, 0x38, 0x20, 0x6C, 0x6F, 0x6F, 0x6B, 0x61, 0x68, 0x65, 0x61, 0x64,
      0x5F, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x73, 0x3D, 0x32, 0x20, 0x73, 0x6C, 0x69, 0x63, 0x65, 0x64, 0x5F, 0x74, 0x68,
      0x72, 0x65, 0x61, 0x64, 0x73, 0x3D, 0x30, 0x20, 0x6E, 0x72, 0x3D, 0x30, 0x20, 0x64, 0x65, 0x63, 0x69, 0x6D, 0x61, 0x74,
      0x65, 0x3D, 0x31, 0x20, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x6C, 0x61, 0x63, 0x65, 0x64, 0x3D, 0x30, 0x20, 0x62, 0x6C, 0x75,
      0x72, 0x61, 0x79, 0x5F, 0x63, 0x6F, 0x6D, 0x70, 0x61, 0x74, 0x3D, 0x30, 0x20, 0x73, 0x74, 0x69, 0x74, 0x63, 0x68, 0x61,
      0x62, 0x6C, 0x65, 0x3D, 0x31, 0x20, 0x63, 0x6F, 0x6E, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6E, 0x65, 0x64, 0x5F, 0x69, 0x6E,
      0x74, 0x72, 0x61, 0x3D, 0x30, 0x20, 0x62, 0x66, 0x72, 0x61, 0x6D, 0x65, 0x73, 0x3D, 0x30, 0x20, 0x77, 0x65, 0x69, 0x67,
      0x68, 0x74, 0x70, 0x3D, 0x30, 0x20, 0x6B, 0x65, 0x79, 0x69, 0x6E, 0x74, 0x3D, 0x69, 0x6E, 0x66, 0x69, 0x6E, 0x69, 0x74,
      0x65, 0x20, 0x6B, 0x65, 0x79, 0x69, 0x6E, 0x74, 0x5F, 0x6D, 0x69, 0x6E, 0x3D, 0x33, 0x30, 0x20, 0x73, 0x63, 0x65, 0x6E,
      0x65, 0x63, 0x75, 0x74, 0x3D, 0x34, 0x30, 0x20, 0x69, 0x6E, 0x74, 0x72, 0x61, 0x5F, 0x72, 0x65, 0x66, 0x72, 0x65, 0x73,
      0x68, 0x3D, 0x30, 0x20, 0x72, 0x63, 0x5F, 0x6C, 0x6F, 0x6F, 0x6B, 0x61, 0x68, 0x65, 0x61, 0x64, 0x3D, 0x34, 0x30, 0x20,
      0x72, 0x63, 0x3D, 0x32, 0x70, 0x61, 0x73, 0x73, 0x20, 0x6D, 0x62, 0x74, 0x72, 0x65, 0x65, 0x3D, 0x31, 0x20, 0x62, 0x69,
      0x74, 0x72, 0x61, 0x74, 0x65, 0x3D, 0x33, 0x30, 0x30, 0x20, 0x72, 0x61, 0x74, 0x65, 0x74, 0x6F, 0x6C, 0x3D, 0x31, 0x2E,
      0x30, 0x20, 0x71, 0x63, 0x6F, 0x6D, 0x70, 0x3D, 0x30, 0x2E, 0x36, 0x30, 0x20, 0x71, 0x70, 0x6D, 0x69, 0x6E, 0x3D, 0x35,
      0x20, 0x71, 0x70, 0x6D, 0x61, 0x78, 0x3D, 0x36, 0x39, 0x20, 0x71, 0x70, 0x73, 0x74, 0x65, 0x70, 0x3D, 0x34, 0x20, 0x63,
      0x70, 0x6C, 0x78, 0x62, 0x6C, 0x75, 0x72, 0x3D, 0x32, 0x30, 0x2E, 0x30, 0x20, 0x71, 0x62, 0x6C, 0x75, 0x72, 0x3D, 0x30,
      0x2E, 0x35, 0x20, 0x76, 0x62, 0x76, 0x5F, 0x6D, 0x61, 0x78, 0x72, 0x61, 0x74, 0x65, 0x3D, 0x33, 0x33, 0x30, 0x20, 0x76,
      0x62, 0x76, 0x5F, 0x62, 0x75, 0x66, 0x73, 0x69, 0x7A, 0x65, 0x3D, 0x33, 0x36, 0x30, 0x20, 0x6E, 0x61, 0x6C, 0x5F, 0x68,
      0x72, 0x64, 0x3D, 0x6E, 0x6F, 0x6E, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x6C, 0x65, 0x72, 0x3D, 0x30, 0x20, 0x69, 0x70, 0x5F,
      0x72, 0x61, 0x74, 0x69, 0x6F, 0x3D, 0x31, 0x2E, 0x34, 0x30, 0x20, 0x61, 0x71, 0x3D, 0x31, 0x3A, 0x31, 0x2E
    ];

    let nal_unit = NALUnit::parse(&mdat_data, 8, 4).unwrap();
    assert_eq!(nal_unit.get_ref_idc(), 0);
    assert_eq!(nal_unit.get_unit_type(), 6);
    assert_eq!(nal_unit.size, 738);
  }
}