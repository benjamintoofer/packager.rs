use std::str;

use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}, iso_box::{ IsoBox, IsoFullBox, find_box }};
use crate::util;

static CLASS: &str = "MVHD";

#[derive(Debug, Eq)]
pub struct MVHD {
  size: u32,
  box_type: String,
  version: u8,
  creation_time: u64,
  modification_time: u64,
  timescale: u32,
  duration: u64,

  // TODO (benjamintoofer@realeyes.com): Implement parsing for these
  // rate: u32,
  // volume: u16,
  // next_track_ID: u32
}

impl PartialEq for MVHD {
  fn eq(&self, other: &Self) -> bool {
        self.size == other.size &&
        self.timescale == other.timescale &&
        self.creation_time == other.creation_time &&
        self. modification_time == other.modification_time &&
        self.duration == other.duration &&
        self.version == other.version &&
        self.box_type.eq(&other.box_type)
  }
}

impl IsoBox for MVHD {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for MVHD {
  fn get_version(&self) -> u8 {
    self.version
  }

  fn get_flags(&self) -> u32 {
    0u32
  }
}

impl MVHD {
  pub fn get_creation_time(&self) -> u64 {
    self.creation_time
  }

  pub fn get_modification_time(&self) -> u64 {
    self.modification_time
  }

  pub fn get_timescale(&self) -> u32 {
    self.timescale
  }

  pub fn get_duration(&self) -> u64 {
    self.duration
  }
}

impl MVHD {
  pub fn parse(mp4: &[u8]) -> Result<MVHD, CustomError> {
    let mvhd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("mvhd", 8, moov));

    if let Some(mvhd_data) = mvhd_option {
      Ok(MVHD::parse_mvhd(mvhd_data)?)
    } else {
       Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn parse_mvhd(mvhd_data: &[u8]) -> Result<MVHD, CustomError> {
    let mut start = 0usize;

    // Parse size
    let size = util::get_u32(mvhd_data, start)?;

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(mvhd_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!("{}", err),
    };

    // Parse version
    start = end;
    let version = util::get_u8(mvhd_data, start)?;

    // Parse creation_time
    start = start + 4;
    let creation_time: u64;
    if version == 0 {
      creation_time = u64::from(util::get_u32(mvhd_data, start)?);
      start = start + 4;

    } else {
      creation_time = util::get_u64(mvhd_data, start)?;
      start = start + 8;
    }

    // Parse modification_time
    let modification_time: u64;
    if version == 0 {
      modification_time = u64::from(util::get_u32(mvhd_data, start)?);
      start = start + 4;
    } else {
      modification_time = util::get_u64(mvhd_data, start)?;
      start = start + 8;
    }

    // Parse timescale
    let timescale = util::get_u32(mvhd_data, start)?;

    // Parse duration
    start = start + 4;
    let duration: u64;
    if version == 0 {
      duration = u64::from(util::get_u32(mvhd_data, start)?);

    } else {
      duration = util::get_u64(mvhd_data, start)?;
    }
    
    Ok(MVHD{
      size: size,
      box_type: box_type,
      version: version,
      creation_time: creation_time,
      modification_time: modification_time,
      timescale: timescale,
      duration: duration
    })
  }
}

pub struct MVHDBuilder {
  timescale: u32,
}

impl MVHDBuilder {
  pub fn create_builder() -> MVHDBuilder {
    MVHDBuilder{
      timescale: 0,
    }
  }

  pub fn timescale(mut self, timescale: u32) -> MVHDBuilder {
    self.timescale = timescale;
    self
  }

  pub fn build(&self) -> Vec<u8> {
    let timescale_array = util::transform_u32_to_u8_array(self.timescale);
    // Default to version 0; 32 bit values instead of 64 bit
    vec![
      // Size
      0x00, 0x00, 0x00, 0x6C,
      // mvhd
      0x6D, 0x76, 0x68, 0x64,
      // version
      0x00,
      // flag
      0x00, 0x00, 0x00,
      // creation_time
      0x00, 0x00, 0x00, 0x00,
      // modification_time
      0x00, 0x00, 0x00, 0x00,
      // timescale
      timescale_array[3], timescale_array[2], timescale_array[1], timescale_array[0],
      // duration
      0x00, 0x00, 0x00, 0x00,
      // int(32) rate = 0x00010000; typically 1.0
      0x00, 0x01, 0x00, 0x00,
      // int(16) volume = 0x0100; typically, full volume
      0x01, 0x00,
      // reserved
      0x00, 0x00,
      // int(32)[2] reserved
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // int(32)[9] matrix
      0x00, 0x01, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,  0x00, 0x01, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x40, 0x00, 0x00, 0x00,
      // bit(32)[6]  pre_defined
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // next_track_ID
      0x00, 0x00, 0x00, 0x00, // I think this is ok? Just being lazy...
    ]
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_mvhd() {
    let mvhd: [u8; 108] = [
      // Size
      0x00, 0x00, 0x00, 0x6C,
      // mvhd
      0x6D, 0x76, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xE8,
      0x00, 0x00, 0x75, 0x51, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF
    ];
  
    let expected_mvhd: MVHD = MVHD{
      box_type: "mvhd".to_string(),
      size: 108,
      version: 0,
      creation_time: 0,
      modification_time: 0,
      timescale: 1000,
      duration: 30033
    };
    assert_eq!(MVHD::parse_mvhd(&mvhd).unwrap(), expected_mvhd);
  }

  #[test]
  fn test_build_mvhd() {
    let expected_mvhd: [u8; 108] = [
      0x00, 0x00, 0x00, 0x6C,
      0x6D, 0x76, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x01, 0x5F, 0x90,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x01, 0x00, 0x00,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x01, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,  0x00, 0x01, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x40, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    let mvhd = MVHDBuilder::create_builder()
      .timescale(90000)
      .build();

    assert_eq!(mvhd, expected_mvhd);
  }
}