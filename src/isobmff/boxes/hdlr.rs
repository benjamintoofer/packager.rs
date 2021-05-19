use std::str;

use crate::{error::{CustomError, construct_error}, iso_box::{IsoBox, IsoFullBox, find_box}};
use crate::error::error_code::{MajorCode, ISOBMFFMinorCode};
use crate::util;

static CLASS: &str = "HDLR";
// HandlerBox 14496-12; 8.4.3
#[derive(Eq, Debug)]
pub struct HDLR {
  size: u32,
  box_type: String,
  handler_type: u32,
  name: String
}

impl IsoBox for HDLR {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for HDLR {
  fn get_version(&self) -> u8 {
    0u8
  }

  fn get_flags(&self) -> u32 {
    0u32
  }
}

impl PartialEq for HDLR {
  fn eq(&self, other: &Self) -> bool {
      self.size == other.size &&
      self.handler_type == other.handler_type &&
      self.name == other.name
  }
}

// Implement HDLR memeber methods
impl HDLR {
  pub fn get_handler_type(&self) -> u32 {
    self.handler_type
  }
}

// Implement HDLR static methods
impl HDLR {
  pub fn parse(mp4: &[u8]) -> Result<HDLR, CustomError> {
    let hdlr_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("mdia", 8, trak))
      .and_then(|mdia|find_box("hdlr", 8, mdia));
    
    if let Some(hdlr_data) = hdlr_option {
      Ok(HDLR::parse_hdlr(hdlr_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn parse_hdlr(hdlr_data: &[u8]) -> Result<HDLR, CustomError> {
    let mut start = 0usize;
    // Parse size
    let size = util::get_u32(hdlr_data, start).unwrap();
      // .expect(format!("{}.parse_hdlr.size: cannot get u32 from start = {}", CLASS, start).as_ref());

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(hdlr_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!("{}", err),
    };


    // Skip version, flag, and 32 bit predfined
    start = start + 12;
    // Parse handler type
    let handler_type = util::get_u32(hdlr_data, start)?;

    // Skip 3 * 32 bit reserved
    start = start + 12 + 4;

    // Parse name
    let mut name = String::from("");
    while hdlr_data[start] != 0 {
      if !hdlr_data[start].is_ascii() {
        return Err(construct_error(
          MajorCode::ISOBMFF,
          Box::new(ISOBMFFMinorCode::PARSE_BOX_ERROR),
          "Handler name; character is not an ascii value".to_string(),
          file!(),
          line!()));
      }
      let character = hdlr_data[start] as char;
      name.push(character);
      start = start + 1;
    }
    Ok(HDLR {
      box_type: box_type,
      size: size,
      handler_type: handler_type,
      name: name
    })
  }
}

#[cfg(test)]
mod tests {

    use super::*;

  #[test]
  fn test_parse_hdlr() {
    let hdlr: [u8; 53] = [
      // Size
      0x00, 0x00, 0x00, 0x35,
      // hdlr
      0x68, 0x64, 0x6C, 0x72,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x76, 0x69, 0x64, 0x65, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x42, 0x65, 0x6E, 0x74, 0x6F, 0x34, 0x20, 0x56,
      0x69, 0x64, 0x65, 0x6F, 0x20, 0x48, 0x61, 0x6E, 
      0x64, 0x6C, 0x65, 0x72, 0x00
    ];
  
    let expected_hdlr: HDLR = HDLR{
      box_type: "hdlr".to_string(),
      size: 53,
      handler_type: 0x76696465,
      name: "Bento4 Video Handler".to_string(),
    };
    assert_eq!(HDLR::parse_hdlr(&hdlr).unwrap(), expected_hdlr);
  }

}
