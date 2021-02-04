use crate::iso_box::{IsoBox, IsoFullBox, find_box};
use crate::util;

// SampleDescriptionBox 14496-12; 8.5.2
#[derive(Eq)]
pub struct STSD {
  size: u32,
  box_type: String,
  entry_count: u32
}

impl IsoBox for STSD {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for STSD {
  fn get_version(&self) -> u8 {
    0u8
  }

  fn get_flags(&self) -> u32 {
    0u32
  }
}

impl PartialEq for STSD {
  fn eq(&self, other: &Self) -> bool {
      self.size == other.size
  }
}

// Implement STSD memeber methods
impl STSD {
  pub fn get_entry_count(&self) -> u32 {
    self.entry_count
  }
}

// Implement STSD static methods
impl STSD {
  pub fn parse(mp4: &[u8]) -> Result<STSD, String> {
    let stsd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("mdia", 8, trak))
      .and_then(|mdia|find_box("minf", 8, mdia))
      .and_then(|minf|find_box("stbl", 8, minf))
      .and_then(|stbl|find_box("stsd", 8, stbl));
    
    if let Some(stsd_data) = stsd_option {
      Ok(STSD::parse_stsd(stsd_data))
    } else {
      Err("unable to find the sidx".to_string())
    }
  }

  fn parse_stsd(stsd_data: &[u8]) -> STSD {

    STSD {
      box_type: "".to_string(),
      size: 0,
      entry_count: 0
    }
  }
}