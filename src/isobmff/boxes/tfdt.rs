use std::str;

use crate::iso_box::{IsoBox, IsoFullBox, find_box};
use crate::util;

#[derive(Debug)]
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

// Implement TFDT memeber methods
impl TFDT {
  pub fn get_base_media_decode_time(&self) -> u64 {
    self.base_media_decode_time
  }
}

// Implement TFDT static methods
impl TFDT {
  pub fn parse(moof: &[u8]) -> Result<TFDT, String> {
    let tfdt_option = find_box("traf", 8, moof)
      .and_then(|traf|find_box("tfdt", 8, traf));

    if let Some(tfdt_data) = tfdt_option {
      let mut start = 0;
      let mut end = start + 4;

      // Parse size
      let size = util::get_u32(tfdt_data, start, end)
        .expect(format!("TFDT.parse.size: cannot get u32 from start = {}; end = {}",start, end).as_ref());

      start = end;
      end = start + 4;
      let box_type = str::from_utf8(tfdt_data[start..end].as_ref()); 
      
      let box_type= match box_type {
        Ok(box_type_str) => String::from(box_type_str),
        Err(err) => panic!(err),
      };

      // Parse version
      start = end;
      end = start + 1;
      let version = util::get_u8(tfdt_data, start, end)
        .expect(format!("TFDT.parse.version: cannot get u32 from start = {}; end = {}",start, end).as_ref());

      // Parse base_media_decode_time
      start = end + 3;
      let base_media_decode_time: u64;
      if version == 0 {
        end = start + 4;
          base_media_decode_time = u64::from(util::get_u32(tfdt_data, start, end)
          .expect(format!("TFDT.parse.base_media_decode_time: cannot get u32 from start = {}; end = {}",start, end).as_ref()));
      } else {
        end = start + 8;
          base_media_decode_time = util::get_u64(tfdt_data, start, end)
          .expect(format!("TFDT.parse.base_media_decode_time: cannot get u64 from start = {}; end = {}",start, end).as_ref());
      }
      return Ok(TFDT {
        box_type: box_type,
        size: size,
        base_media_decode_time: base_media_decode_time,
        version: version
      })
    } else {
      Err("unable to find the tfdt".to_string())
    }
  }
}