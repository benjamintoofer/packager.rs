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
      Err(err) => panic!("{}", err),
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

pub struct TFHDBuilder {
  track_id: usize,
  base_data_offset: Option<usize>,
  sample_description_index: Option<u32>,
  sample_duration: Option<u32>,
  sample_size: Option<u32>,
  sample_flags: Option<u32>,
  duration_is_empty: bool,
  default_base_is_moof: bool,
}

impl TFHDBuilder {
  pub fn create_builder() -> TFHDBuilder {
    TFHDBuilder{
      track_id: 1,
      base_data_offset: None,
      sample_description_index: None,
      sample_duration: None,
      sample_size: None,
      sample_flags: None,
      duration_is_empty: false,
      default_base_is_moof: true,
    }
  }

  pub fn track_id(mut self, track_id: usize) -> TFHDBuilder {
    self.track_id = track_id;
    self
  }

  pub fn base_data_offset(mut self, base_data_offset: usize) -> TFHDBuilder {
    self.base_data_offset = Some(base_data_offset);
    self
  }

  pub fn sample_description_index(mut self, sample_description_index: u32) -> TFHDBuilder {
    self.sample_description_index = Some(sample_description_index);
    self
  }

  pub fn sample_duration(mut self, sample_duration: Option<u32>) -> TFHDBuilder {
    self.sample_duration = sample_duration;
    self
  }

  pub fn sample_size(mut self, sample_size: u32) -> TFHDBuilder {
    self.sample_size = Some(sample_size);
    self
  }

  pub fn sample_flags(mut self, sample_flags: u32) -> TFHDBuilder {
    self.sample_flags = Some(sample_flags);
    self
  }

  /// Something
  fn generate_flag(&self) -> (u32, Vec<u8>) {
    let mut flag: u32 = 0x00000000;
    let mut data: Vec<u8> = vec![];
    // base-data-offset-present
    if let Some(base_data_offset) = self.base_data_offset {
      flag += 0x000001;
      let array_val = util::transform_usize_to_u8_array(base_data_offset);
      data = [
        data,
        vec![
          array_val[7],array_val[6],array_val[5], array_val[4],
          array_val[3],array_val[2],array_val[1], array_val[0],
        ]
      ].concat()
    }

    // sample-description-index-present
    if let Some(sample_description_index) = self.sample_description_index {
      flag += 0x000002;
      let array_val = util::transform_u32_to_u8_array(sample_description_index);
      data = [
        data,
        vec![
          array_val[3],array_val[2],array_val[1], array_val[0],
        ]
      ].concat()
    }
    // default-sample-duration-present
    if let Some(sample_duration) = self.sample_duration {
      flag += 0x000008;
      let array_val = util::transform_u32_to_u8_array(sample_duration);
      data = [
        data,
        vec![
          array_val[3],array_val[2],array_val[1], array_val[0],
        ]
      ].concat()
    }
    // default-sample-size-present
    if let Some(sample_size) = self.sample_size {
      flag += 0x000010;
      let array_val = util::transform_u32_to_u8_array(sample_size);
      data = [
        data,
        vec![
          array_val[3],array_val[2],array_val[1], array_val[0],
        ]
      ].concat()
    }
    // default-sample-flags-present
    if let Some(sample_flags) = self.sample_flags {
      flag += 0x000020;
      let array_val = util::transform_u32_to_u8_array(sample_flags);
      data = [
        data,
        vec![
          array_val[3],array_val[2],array_val[1], array_val[0],
        ]
      ].concat()
    }
    // duration-is-empty
    if self.duration_is_empty {
      flag += 0x010000;
    }

     // default-base-is-moof
    if self.default_base_is_moof {
      flag += 0x020000;
    }
    (flag, data)
  }

  pub fn build(&self) -> Vec<u8> {
    let track_id_array = util::transform_usize_to_u8_array(self.track_id);
    // let sample_duration_array = util::transform_usize_to_u8_array(self.sample_duration);
    let (flag, data) = self.generate_flag();
    let flag_array = util::transform_u32_to_u8_array(flag);
    let size = 
      12 + // header
      4 + // track id
      data.len();
    let size_array = util::transform_usize_to_u8_array(size);
    [
      vec![
        // size
        size_array[3], size_array[2], size_array[1], size_array[0],
        // tfhd
        0x74, 0x66, 0x68, 0x64,
        // version
        0x00,
        // flag
        flag_array[2], flag_array[1], flag_array[0],
        // track id
        track_id_array[3], track_id_array[2], track_id_array[1], track_id_array[0]
      ],
      data
    ].concat()
  }
}

/** CURRRENT TFHD EXAMPLE
track_id                                  => 1
0x000002 sample-description-index-present => 1
0x000008 default-sample-duration-present  => 1 
0x000020 default-sample-flags-present     => 0x01010000
rese  is_lead | sample_depends_on | sample_is_depended_on | sample_has_redundancy | sample_padding_value | sample_is_non_sync_sample  |  sample_degradation_priority
0000    00            01                  00                      00                      000                         1                         00000000 00000000
0x020000 default-base-is-moof:
*/
/**
0x000001 base-data-offset-present = NO
0x000002 sample-description-index-present = ??
0x000008 default-sample-duration-present = ??
0x020000 default-base-is-moof = YES
*/

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_tfhd() {
    let tfhd: [u8; 28] = [
      // size
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

  #[test]
  fn test_build_tfhd_minimum() {
    let expected_tfhd: [u8; 16] = [
      0x00, 0x00, 0x00, 0x10,
      0x74, 0x66, 0x68, 0x64,
      0x00, 0x02, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01,
    ];
    let tfhd = TFHDBuilder::create_builder()
      .track_id(1)
      // .sample_duration(1)
      .build();
    assert_eq!(tfhd, expected_tfhd);
  }

  #[test]
  fn test_build_tfhd_maximum() {
    let expected_tfhd: [u8; 40] = [
      0x00, 0x00, 0x00, 0x28,
      0x74, 0x66, 0x68, 0x64,
      0x00, 0x02, 0x00, 0x3B,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x02,
      0x00, 0x00, 0x00, 0x03,
      0x00, 0x00, 0x00, 0x04,
      0x00, 0x00, 0x00, 0x05,
    ];
    let tfhd = TFHDBuilder::create_builder()
      .track_id(1)
      .base_data_offset(1)
      .sample_description_index(2)
      .sample_duration(Some(3))
      .sample_size(4)
      .sample_flags(5)
      .build();
    assert_eq!(tfhd, expected_tfhd);
  }
}