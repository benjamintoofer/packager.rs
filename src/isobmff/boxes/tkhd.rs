use std::str;

use crate::iso_box::find_box;
use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}};
use crate::util;

static CLASS: &str = "TKHD";

#[derive(Debug, Eq)]
pub struct TKHDReader {
  data: Vec<u8>,
  size: u32,
  box_type: String,
  version: u8,
  flags: u32,

  creation_time: Option<u64>,
  modification_time: Option<u64>,
  track_id: Option<u32>,
  duration: Option<u64>,
  // NOTE (benjamintoofer@gmail.com): width and height is read as u32 but it's a 16.16 fixed point value. Meaning we must
  // divide the value by 65536.0 to get it's interepted floating point value.
  width: Option<u32>,
  height: Option<u32>,
}

impl PartialEq for TKHDReader {
  fn eq(&self, other: &Self) -> bool {
    self.size == other.size &&
    self.flags == other.flags &&
    self.height == other.height &&
    self.width == other.width &&
    self.duration == other.duration &&
    self.track_id == other.track_id &&
    self.creation_time == other.creation_time &&
    self.modification_time == other.modification_time
  }
}

impl TKHDReader {
  pub fn parse(mp4: &[u8]) -> Result<TKHDReader, CustomError> {
    let tkhd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("tkhd", 8, trak));
    
    if let Some(tkhd_data) = tkhd_option {
      Ok(TKHDReader::get_reader(tkhd_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }
  pub fn get_reader(tkhd_data: &[u8]) -> Result<TKHDReader, CustomError> {
    let mut start = 0usize;
    // Parse size
    let size = util::get_u32(tkhd_data, start)?;

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(tkhd_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!("{}", err),
    };

    // Parse version
    start = end;
    let version = util::get_u8(tkhd_data, start)?;
    Ok(TKHDReader{
      data: tkhd_data.to_vec(),
      size,
      box_type,
      version,
      flags: 0,
      creation_time: Option::None,
      modification_time: Option::None,
      track_id: Option::None,
      duration: Option::None,
      width: Option::None,
      height: Option::None
    })
  }
  // Instance methods
  pub fn get_creation_time(&mut self) -> Result<u64, CustomError> {
    if self.creation_time.is_none() {
      let offset = 12usize;
      if self.version == 0 {
        self.creation_time = Option::Some(u64::from(util::get_u32(&self.data, offset)?));
      } else {
        self.creation_time = Option::Some(util::get_u64(&self.data, offset)?);
      }
    }
    self.creation_time.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to get creation time", CLASS),
        file!(),
        line!()))
  }

   pub fn get_modification_time(&mut self) -> Result<u64, CustomError> {
    if self.modification_time.is_none() {
      if self.version == 0 {
        self.modification_time = Option::Some(u64::from(util::get_u32(&self.data, 16)?));
      } else {
        self.modification_time = Option::Some(util::get_u64(&self.data, 20)?);
      }
    }
    self.modification_time.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to get modification time", CLASS),
        file!(),
        line!()))
  }

  pub fn get_track_id(&mut self) -> Result<u32, CustomError> {
    if self.track_id.is_none() {
      if self.version == 0 {
        self.track_id = Option::Some(util::get_u32(&self.data, 20)?);
      } else {
        self.track_id = Option::Some(util::get_u32(&self.data, 28)?);
      }
    }
    self.track_id.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to get track id", CLASS),
        file!(),
        line!()))
  }

  pub fn get_duration(&mut self) -> Result<u64, CustomError> {
    if self.duration.is_none() {
      if self.version == 0 {
        self.duration = Option::Some(u64::from(util::get_u32(&self.data, 28)?));
      } else {
        self.duration = Option::Some(util::get_u64(&self.data, 36)?);
      }
    }
    self.duration.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to get duration", CLASS),
        file!(),
        line!()))
  }
  
  pub fn get_width(&mut self) -> Result<u32, CustomError> {
    if self.width.is_none() {
      if self.version == 0 {
        self.width = Option::Some(util::get_u32(&self.data, 84)?);
      } else {
        self.width = Option::Some(util::get_u32(&self.data, 96)?);
      }
    }
    self.width.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to get width", CLASS),
        file!(),
        line!()))
  }

  pub fn get_height(&mut self) -> Result<u32, CustomError> {
    if self.height.is_none() {
      if self.version == 0 {
        self.height = Option::Some(util::get_u32(&self.data, 88)?);
      } else {
        self.height = Option::Some(util::get_u32(&self.data, 100)?);
      }
    }
    self.height.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to get height", CLASS),
        file!(),
        line!()))
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_tkhd() {

    let tkhd: [u8; 92] = [
      // Size
      0x00, 0x00, 0x00, 0x5C,
      //tkhd
      0x74, 0x6B, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x75, 0x51, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x01, 0xE0, 0x00, 0x00, 0x01, 0x0E, 0x00, 0x00
    ];
  
    let expected_tkhd: TKHDReader = TKHDReader {
      data: (&tkhd).to_vec(),
      box_type: "tkhd".to_string(),
      size: 92,
      version: 0,
      flags: 0,
      creation_time: Option::None,
      modification_time: Option::None,
      duration: Option::None,
      track_id: Option::None,
      width: Option::None,
      height: Option::None
    };

    let mut tkhd_reader = TKHDReader::get_reader(&tkhd).unwrap();
    assert_eq!(tkhd_reader, expected_tkhd);
    assert_eq!(tkhd_reader.get_creation_time().unwrap(), 0);
    assert_eq!(tkhd_reader.get_modification_time().unwrap(), 0);
    assert_eq!(tkhd_reader.get_track_id().unwrap(), 1);
    assert_eq!(tkhd_reader.get_duration().unwrap(), 30033);
    assert_eq!(tkhd_reader.get_width().unwrap() as f32 / 65536.0, 480.0);
    assert_eq!(tkhd_reader.get_height().unwrap() as f32 / 65536.0, 270.0);
  }
}