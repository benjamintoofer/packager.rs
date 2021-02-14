use std::str;

use crate::iso_box::{IsoBox, IsoFullBox, find_box};
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
 
}

// Implement HDLR static methods
impl HDLR {
  pub fn parse(mp4: &[u8]) -> Result<HDLR, String> {
    let hdlr_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("mdia", 8, trak))
      .and_then(|mdia|find_box("hdlr", 8, mdia));
    
    if let Some(hdlr_data) = hdlr_option {
      Ok(HDLR::parse_hdlr(hdlr_data))
    } else {
      Err("unable to find the hdlr".to_string())
    }
  }

  fn parse_hdlr(hdlr_data: &[u8]) -> HDLR {
    let mut start = 0usize;
    // Parse size
    let size = util::get_u32(hdlr_data, start)
      .expect(format!("{}.parse_hdlr.size: cannot get u32 from start = {}", CLASS, start).as_ref());

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(hdlr_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!(err),
    };


    // Skip version, flag, and 32 bit predfined
    start = start + 12;
    // Parse handler type
    let handler_type = util::get_u32(hdlr_data, start)
      .expect(format!("{}.parse_hdlr.handler_type: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());

    // Skip 3 * 32 bit reserved
    start = start + 12 + 4;

    // Parse name
    let mut name = String::from("");
    while hdlr_data[start] != 0 {
      if !hdlr_data[start].is_ascii() {
        // Error("")
        todo!()
      }
      let character = hdlr_data[start] as char;
      name.push(character);
      start = start + 1;
    }
    HDLR {
      box_type: box_type,
      size: size,
      handler_type: handler_type,
      name: name
    }
  }
}

#[cfg(test)]
mod tests {

  use std::fs;

    use super::*;

  #[test]
  fn test_parse_hdlr() {
    let file_path = "./assets/v_frag.mp4";
  
    let expected_mvhd: HDLR = HDLR{
      box_type: "hdlr".to_string(),
      size: 53,
      handler_type: 0x76696465,
      name: "Bento4 Video Handler".to_string(),
    };
    let mp4_file = fs::read(file_path);
    if let Ok(mp4) = mp4_file {
      assert_eq!(HDLR::parse(&mp4).unwrap(), expected_mvhd);
    } else {
      panic!("mp4 file {:} cannot be opened", file_path);
    }
  }

}
