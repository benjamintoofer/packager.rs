use std::str;
use std::convert::TryFrom;

use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}, iso_box::{IsoBox, IsoFullBox, find_box}};
use crate::util;

static CLASS: &str = "STSD";

// SampleDescriptionBox 14496-12; 8.5.2
#[derive(Eq, Debug)]
pub struct STSD<'a> {
  size: u32,
  box_type: String,
  entry_count: u32,
  sample_entries: &'a [u8]
}

impl<'a> IsoBox for STSD<'a> {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl<'a> IsoFullBox for STSD<'a> {
  fn get_version(&self) -> u8 {
    0u8
  }

  fn get_flags(&self) -> u32 {
    0u32
  }
}

impl<'a> PartialEq for STSD<'a> {
  fn eq(&self, other: &Self) -> bool {
      self.size == other.size &&
      self.entry_count == other.entry_count
  }
}

// Implement STSD member methods
impl<'a> STSD<'a> {
  pub fn get_entry_count(&self) -> u32 {
    self.entry_count
  }

  pub fn get_samples_length(&self) -> usize {
    self.sample_entries.len()
  }
}

// Implement STSD static methods
impl<'a> STSD<'a> {
  pub fn parse(mp4: &[u8]) -> Result<STSD, CustomError> {
    let stsd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("mdia", 8, trak))
      .and_then(|mdia|find_box("minf", 8, mdia))
      .and_then(|minf|find_box("stbl", 8, minf))
      .and_then(|stbl|find_box("stsd", 8, stbl));
    
    if let Some(stsd_data) = stsd_option {
      Ok(STSD::parse_stsd(stsd_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn read_sample_entry(&self, box_type: &str) -> Result<&[u8], CustomError> {
    // TODO (benjamintoofer@gmail.com): This needs to be redone. Need to iterate through all entries.
    let sample_entry_data = find_box(box_type, 0, self.sample_entries);

    if let Some(sample_entry_data) = sample_entry_data {
      Ok(sample_entry_data)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to sample entry", box_type),
        file!(),
        line!()))
    }
  }

  fn parse_stsd(stsd_data: &'a [u8]) -> Result<STSD, CustomError> {
    let mut start = 0usize;

    // Parse size
    let size = util::get_u32(stsd_data, start)?;

    start = start + 4;
    let mut end = start + 4;
    let box_type = str::from_utf8(stsd_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!(err),
    };

    // Parse entry count
    start = start + 8;
    let entry_count = util::get_u32(stsd_data, start)?;
    
    start = start + 4;
    end = usize::try_from(size).expect("cannot convert u32 (num) to usize");
    let entries: &[u8] = stsd_data[start..end].as_ref();

    Ok(STSD {
      box_type,
      size,
      entry_count,
      sample_entries: entries
    })
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::fs;

  #[test]
  fn test_parse_stsd() {
    let file_path = "./assets/v_frag.mp4";
  
    let expected_stsd: STSD = STSD{
      box_type: "stsd".to_string(),
      size: 166,
      entry_count: 1,
      sample_entries: &[], // We dont compare this
    };
    let mp4_file = fs::read(file_path);
    if let Ok(mp4) = mp4_file {
      assert_eq!(STSD::parse(&mp4).unwrap(), expected_stsd);
    } else {
      panic!("mp4 file {:} cannot be opened", file_path);
    }
  }
}