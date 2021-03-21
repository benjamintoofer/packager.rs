use std::convert::TryInto;

use crate::error::{CustomError, construct_error};
use crate::error::error_code::{MajorCode, NalMinorCode};
use crate::util;

pub struct NALUnit<'a> {
  pub size: usize,
  nal_data: &'a [u8]
}

impl<'a> NALUnit<'a> {
  pub fn get_ref_idc(&self) -> u8 {
    (self.nal_data[0] & 0x60) >> 5
  }

  pub fn get_unit_type(&self) -> u8 {
    self.nal_data[0] & 0x1F
  }
}

impl<'a> NALUnit<'a> {
  pub fn parse(mdat: &[u8], offset: usize, nal_unit_length: u8) -> Result<NALUnit, CustomError> {
    let nal_size = NALUnit::get_nal_unit_size(mdat, offset, nal_unit_length)?;
    let offset_without_nal_length = offset + nal_unit_length as usize;
    let nal_data = mdat[offset_without_nal_length..(offset + nal_size)].as_ref();

    Ok(NALUnit {
      size: nal_size,
      nal_data
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

  pub fn parse_sei() {
    // TODO
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