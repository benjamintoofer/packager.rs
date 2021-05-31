use std::str;

use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}};
use crate::iso_box::find_box;
use crate::util;

static CLASS: &str = "MDHD";

#[derive(Debug, Eq)]
pub struct MDHDReader {
  data: Vec<u8>,
  size: u32,
  box_type: String,
  version: u8,
  flags: u32,

  creation_time: Option<u64>,
  modification_time: Option<u64>,
  timescale: Option<u32>,
  duration: Option<u64>,

  language: Option<u16>,
}

impl PartialEq for MDHDReader {
  fn eq(&self, other: &Self) -> bool {
    self.size == other.size &&
    self.flags == other.flags &&
    self.duration == other.duration &&
    self.creation_time == other.creation_time &&
    self.modification_time == other.modification_time
  }
}

impl MDHDReader {
  pub fn parse(mp4: &[u8]) -> Result<MDHDReader, CustomError> {
    let mdhd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("mdia", 8, trak))
      .and_then(|mdia|find_box("mdhd", 8, mdia));
    
    if let Some(mdhd_data) = mdhd_option {
      Ok(MDHDReader::get_reader(mdhd_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn get_reader(mdhd_data: &[u8]) -> Result<MDHDReader, CustomError> {
    let mut start = 0usize;
    // Parse size
    let size = util::get_u32(mdhd_data, start)?;

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(mdhd_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!("{}", err),
    };

    // Parse version
    start = end;
    let version = util::get_u8(mdhd_data, start)?;
    Ok(MDHDReader{
      data: mdhd_data.to_vec(),
      size,
      box_type,
      version,
      flags: 0,
      creation_time: Option::None,
      modification_time: Option::None,
      duration: Option::None,
      timescale: Option::None,
      language: Option::None
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
        Box::new(ISOBMFFMinorCode::PARSE_BOX_ERROR),
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
        Box::new(ISOBMFFMinorCode::PARSE_BOX_ERROR),
        format!("{}: Unable to get modification time", CLASS),
        file!(),
        line!()))
  }

  pub fn get_timescale(&mut self) -> Result<u32, CustomError> {
   if self.timescale.is_none() {
      if self.version == 0 {
        self.timescale = Option::Some(util::get_u32(&self.data, 20)?);
      } else {
        self.timescale = Option::Some(util::get_u32(&self.data, 28)?);
      }
    }
    self.timescale.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::PARSE_BOX_ERROR),
        format!("{}: Unable to get timescale", CLASS),
        file!(),
        line!()))
  }

  pub fn get_duration(&mut self) -> Result<u64, CustomError> {
    if self.duration.is_none() {
      if self.version == 0 {
        self.duration = Option::Some(u64::from(util::get_u32(&self.data, 24)?));
      } else {
        self.duration = Option::Some(util::get_u64(&self.data, 32)?);
      }
    }
    self.duration.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::PARSE_BOX_ERROR),
        format!("{}: Unable to get duration", CLASS),
        file!(),
        line!()))
  }

  pub fn get_language(&mut self) -> Result<String, CustomError> {
    let mut temp: u16 = 0;
    if self.language.is_none() {
      if self.version == 0 {
        temp = util::get_u16(&self.data, 28)?;
      } else {
        temp = util::get_u16(&self.data, 40)?;
      }
    }
    let mut language_str:[char; 3] = ['a'; 3];
    for i in 0..3 {
      let shifted_val = temp >> (i * 5);
      let mask: u16 = 0b11111;
      let val = ((shifted_val & mask) + 0x60) as u8;
      language_str[2 - i] = val as char;
    }

    Ok(language_str.iter().collect::<String>())
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_mdhd() {

    let mdhd: [u8; 32] = [
      // Size
     0x00, 0x00, 0x00, 0x20,
     // mdhd
     0x6D, 0x64, 0x68, 0x64,
     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x55, 0xC4, 0x00, 0x00
    ];
  
    let expected_mdhd: MDHDReader = MDHDReader {
      data: (&mdhd).to_vec(),
      box_type: "mdhd".to_string(),
      size: 32,
      version: 0,
      flags: 0,
      creation_time: Option::None,
      modification_time: Option::None,
      duration: Option::None,
      timescale: Option::None,
      language: Option::None,
    };

    let mut mdhd_reader = MDHDReader::get_reader(&mdhd).unwrap();
    assert_eq!(mdhd_reader, expected_mdhd);
    assert_eq!(mdhd_reader.get_creation_time().unwrap(), 0);
    assert_eq!(mdhd_reader.get_modification_time().unwrap(), 0);
    assert_eq!(mdhd_reader.get_duration().unwrap(), 0);
    assert_eq!(mdhd_reader.get_timescale().unwrap(), 30);
    assert_eq!(mdhd_reader.get_language().unwrap(), "und");
  }
}