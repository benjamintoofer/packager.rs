use std::str;

use crate::iso_box::{IsoBox, IsoFullBox, find_box};
use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}};
use crate::util;

static CLASS: &str = "TFHD";

#[derive(Debug, Eq)]
pub struct TFHD {
  size: u32,
  box_type: String,
  version: u8,                    // 0
  flags: u32,                     // u24
  track_id: u32,
  base_data_offset: Option<u64>,
  sample_description_index: Option<u32>,
  default_sample_duration: Option<u32>,
  default_sample_size: Option<u32>,
  default_sample_flags: Option<u32>,
  duration_is_empty: bool,
  default_base_is_moof: bool,
}

impl IsoBox for TFHD {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for TFHD {
  fn get_version(&self) -> u8 {
    0u8
  }

  fn get_flags(&self) -> u32 {
    self.flags
  }
}

impl PartialEq for TFHD {
  fn eq(&self, other: &Self) -> bool {
    self.size == other.size &&
    self.flags == other.flags &&
    self.base_data_offset == other.base_data_offset &&
    self.sample_description_index == other.sample_description_index &&
    self.default_sample_duration == other.default_sample_duration &&
    self.default_sample_size == other.default_sample_size &&
    self.default_sample_flags == other.default_sample_flags &&
    self.default_base_is_moof == other.default_base_is_moof &&
    self.duration_is_empty == other.duration_is_empty 
  }
}

impl TFHD {
  pub fn parse(moof: &[u8]) -> Result<TFHD, CustomError> {
    let tfhd_option = find_box("traf", 8, moof)
      .and_then(|traf|find_box("tfhd", 8, traf));
    
    if let Some(tfhd_data) = tfhd_option {
      Ok(TFHD::parse_tfhd(tfhd_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn parse_tfhd(tfhd_data: &[u8]) -> Result<TFHD, CustomError> {
    let mut start = 0usize;

    // Parse size
    let size = util::get_u32(tfhd_data, start)?;

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(tfhd_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!(err),
    };

    // Parse flags
    start = start + 4;
    let flags = util::get_u32(tfhd_data, start)? & 0xFFFFFF;

    start = start + 4;
    let track_id = util::get_u32(tfhd_data, start)?;

    start = start + 4;
    // base-data-offset-present
    let mut base_data_offset: Option<u64> = Option::None;
    if (flags & 0x000001) != 0 {
      base_data_offset = Option::Some(util::get_u64(tfhd_data, start)?);
      start = start + 8;
    }

    // sample-description-index-present
    let mut sample_description_index: Option<u32> = Option::None;
    if (flags & 0x000002) != 0 {
      sample_description_index = Option::Some(util::get_u32(tfhd_data, start)?);
      start = start + 4;
    }

    // default-sample-duration-present
    let mut default_sample_duration: Option<u32> = Option::None;
    if (flags & 0x000008) != 0 {
      default_sample_duration = Option::Some(util::get_u32(tfhd_data, start)?);
      start = start + 4;
    }

    // default-sample-size-present
    let mut default_sample_size: Option<u32> = Option::None;
    if (flags & 0x0000010) != 0 {
      default_sample_size = Option::Some(util::get_u32(tfhd_data, start)?);
      start = start + 4;
    }

    // default-sample-flags-present
    let mut default_sample_flags: Option<u32> = Option::None;
    if (flags & 0x0000020) != 0 {
      default_sample_flags = Option::Some(util::get_u32(tfhd_data, start)?);
      start = start + 4;
    }

    // duration-is-empty
    let duration_is_empty = (flags & 0x010000) != 0;
    let default_base_is_moof = (flags & 0x020000) != 0;
    
    Ok(TFHD{
      size,
      box_type,
      version: 0,
      flags,
      track_id,
      base_data_offset,
      sample_description_index,
      default_sample_duration,
      default_sample_size,
      default_sample_flags,
      duration_is_empty,
      default_base_is_moof
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_tfhd() {
    let tfhd: [u8; 28] = [
      // Size
      0x00, 0x00, 0x00, 0x1C,
      // tfhd
      0x74, 0x66, 0x68, 0x64,
      0x00, 0x02, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
      0x01, 0x01, 0x00, 0x00
    ];
    let expected_tfhd: TFHD = TFHD{
      box_type: "tfhd".to_string(),
      size: 28,
      version: 0,
      flags: 0x2002A,
      track_id: 1,
      sample_description_index: Option::Some(1),
      default_sample_flags: Option::Some( 0x1010000),
      base_data_offset: Option::None,
      default_sample_duration: Option::Some(1),
      default_sample_size: Option::None,
      default_base_is_moof: true,
      duration_is_empty: false
    };
    assert_eq!(TFHD::parse_tfhd(&tfhd).unwrap(), expected_tfhd);
  }
}