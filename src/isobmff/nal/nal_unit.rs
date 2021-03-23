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
  pub fn parse(mdat: &[u8], offset: usize, nal_unit_length: u8) -> Result<NALUnit, CustomError> {
    let nal_size = NALUnit::get_nal_unit_size(mdat, offset, nal_unit_length)?;
    let offset_without_nal_length = offset + nal_unit_length as usize;
    let nal_data = mdat[offset_without_nal_length..(offset + nal_size)].as_ref();
    let mut rbsp_bytes: Vec<u8> = vec![];
    for mut i in 0..nal_data.len() {
      if 
        i + 2 < nal_data.len() && 
        nal_data[i] == 0x0 &&
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
        Box::new(NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH),
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
  use std::fs;
  use crate::iso_box::find_box;
  #[test]
  fn test_nal_unit_parsing() {
    let file_path = "./assets/v_frag.mp4";
  
    let mp4_file = fs::read(file_path);
    if let Ok(mp4) = mp4_file {
      let mdat_data = find_box("mdat", 0, mp4.as_ref()).unwrap();
      let nal_unit = NALUnit::parse(&mdat_data, 8, 4).unwrap();
      assert_eq!(nal_unit.get_ref_idc(), 0);
      assert_eq!(nal_unit.get_unit_type(), 6);
      assert_eq!(nal_unit.size, 738);
    } else {
      panic!("mp4 file {:} cannot be opened", file_path);
    }
  }
}