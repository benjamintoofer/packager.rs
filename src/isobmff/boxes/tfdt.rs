use std::str;

use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}, iso_box::{IsoBox, IsoFullBox, find_box}};
use crate::util;

static CLASS: &str = "TFDT";

#[derive(Debug, Eq)]
pub struct TFDT {
  size: u32,
  box_type: String,
  version: u8,
  base_media_decode_time: u64
}

impl IsoBox for TFDT {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for TFDT {
  fn get_version(&self) -> u8 {
    self.version
  }

  fn get_flags(&self) -> u32 {
    0u32
  }
}

impl PartialEq for TFDT {
  fn eq(&self, other: &Self) -> bool {
      self.size == other.size &&
      self.base_media_decode_time == other.base_media_decode_time
  }
}

// Implement TFDT memeber methods
impl TFDT {
  pub fn get_base_media_decode_time(&self) -> u64 {
    self.base_media_decode_time
  }
}

// Implement TFDT static methods
impl TFDT {
  pub fn parse(moof: &[u8]) -> Result<TFDT, CustomError> {
    let tfdt_option = find_box("traf", 8, moof)
      .and_then(|traf|find_box("tfdt", 8, traf));

    if let Some(tfdt_data) = tfdt_option {
      let mut start = 0;
      // Parse size
      let size = util::get_u32(tfdt_data, start)?;

      start = start + 4;
      let end = start + 4;
      let box_type = str::from_utf8(tfdt_data[start..end].as_ref()); 
      
      let box_type= match box_type {
        Ok(box_type_str) => String::from(box_type_str),
        Err(err) => panic!(err),
      };

      // Parse version
      start = start + 4;
      let version = util::get_u8(tfdt_data, start)?;

      // Parse base_media_decode_time
      start = start + 4;
      let base_media_decode_time: u64;
      if version == 0 {
        base_media_decode_time = u64::from(util::get_u32(tfdt_data, start)?);
      } else {
        base_media_decode_time = util::get_u64(tfdt_data, start)?;
      }
      Ok(TFDT {
        box_type: box_type,
        size: size,
        base_media_decode_time: base_media_decode_time,
        version: version
      })
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::fs;

  #[test]
  fn test_parse_tfdt() {
    let file_path = "./assets/v_frag.mp4";
    
    let expected_tfdt: TFDT = TFDT{
      box_type: "tfdt".to_string(),
      size: 20,
      version: 0,
      base_media_decode_time: 0,
    };
    let mp4_file = fs::read(file_path);
    if let Ok(mp4) = mp4_file {
      let moof_data = find_box("moof", 0, mp4.as_ref()).unwrap();
      assert_eq!(TFDT::parse(&moof_data).unwrap(), expected_tfdt);
    } else {
      panic!("mp4 file {:} cannot be opened", file_path);
    }
  }
}