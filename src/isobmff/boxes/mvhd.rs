use std::str;

use crate::iso_box::{ IsoBox, IsoFullBox, find_box };
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
  pub fn parse(mp4: &[u8]) -> Result<MVHD, String> {
    let mvhd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("mvhd", 8, moov));

    if let Some(mvhd_data) = mvhd_option {
      Ok(MVHD::parse_mvhd(mvhd_data))
    } else {
      Err("unable to find the mvhd".to_string())
    }
  }

  fn parse_mvhd(mvhd_data: &[u8]) -> MVHD {
    let mut start = 0usize;

    // Parse size
    let size = util::get_u32(mvhd_data, start)
      .expect(format!("{}.parse_mvhd.size: cannot get u32 from start = {}", CLASS, start).as_ref());

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(mvhd_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!(err),
    };

    // Parse version
    start = end;
    let version = util::get_u8(mvhd_data, start)
      .expect(format!("{}.parse_mvhd.version: cannot get u32 from start = {}",CLASS, start).as_ref());

    // Parse creation_time
    start = start + 3;
    let creation_time: u64;
    if version == 0 {
      creation_time = u64::from(util::get_u32(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.creation_time: cannot get u32 from start = {}",CLASS, start).as_ref()));
      start = start + 4;

    } else {
      creation_time = util::get_u64(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.creation_time: cannot get u64 from start = {}",CLASS, start).as_ref());
      start = start + 8;
    }

    // Parse modification_time
    let modification_time: u64;
    if version == 0 {
      modification_time = u64::from(util::get_u32(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.modification_time: cannot get u32 from start = {}",CLASS, start).as_ref()));
      start = start + 4;
    } else {
      modification_time = util::get_u64(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.modification_time: cannot get u64 from start = {}",CLASS, start).as_ref());
      start = start + 8;
    }

    // Parse timescale
    let timescale = util::get_u32(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.timescale: cannot get u32 from start = {}",CLASS, start).as_ref());

    // Parse duration
    start = start + 4;
    let duration: u64;
    if version == 0 {
      duration = u64::from(util::get_u32(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.duration: cannot get u32 from start = {}",CLASS, start).as_ref()));
      start = start + 4;

    } else {
      duration = util::get_u64(mvhd_data, start)
        .expect(format!("{}.parse_mvhd.duration: cannot get u64 from start = {}",CLASS, start).as_ref());
      start = start + 8;
    }
    
    MVHD{
      size: size,
      box_type: box_type,
      version: version,
      creation_time: creation_time,
      modification_time: modification_time,
      timescale: timescale,
      duration: duration
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::fs;

  #[test]
  fn test_parse_mvhd() {
    let file_path = "./assets/v_frag.mp4";
  
    let expected_mvhd: MVHD = MVHD{
      box_type: "mvhd".to_string(),
      size: 108,
      version: 0,
      creation_time: 0,
      modification_time: 0,
      timescale: 1000,
      duration: 30033
    };
    let mp4_file = fs::read(file_path);
    if let Ok(mp4) = mp4_file {
      assert_eq!(MVHD::parse(&mp4).unwrap(), expected_mvhd);
    } else {
      panic!("mp4 file {:} cannot be opened", file_path);
    }
  }
}