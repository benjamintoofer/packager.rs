use std::str;

use crate::iso_box::{IsoBox, IsoFullBox, find_box};
use crate::util;

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
      self.size == other.size
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
    let mut end = start + 4;

    // Parse size
    let size = util::get_u32(hdlr_data, start, end)
      .expect(format!("HDLR.parse_hdlr.size: cannot get u32 from start = {}; end = {}",start, end).as_ref());

    start = end;
    end = start + 4;
    let box_type = str::from_utf8(hdlr_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!(err),
    };


    // Skip version, flag, and 32 bit predfined
    start = end + 8;

    // Parse handler type
    end = start + 4;
    let handler_type = util::get_u32(hdlr_data, start, end)
      .expect(format!("HDLR.parse_hdlr.handler_type: cannot get u32 from start = {}; end = {}",start, end).as_ref());

    // Skip 3 * 32 bit reserved
    start = end + 12;

    // Parse name
    let mut name = String::from("");
    while hdlr_data[start] != 0 {
      if !hdlr_data[start].is_ascii() {
        // Error("")
      }
      let character = hdlr_data[start] as char;
      println!("{:?}",character);
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

