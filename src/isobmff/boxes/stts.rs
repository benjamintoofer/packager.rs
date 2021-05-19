use std::str;

use crate::error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}};
use crate::util;
use crate::iso_box::find_box;

static CLASS: &str = "STTS";

#[derive(Debug, PartialEq, Eq)]
struct STTSSample {
  sample_count: u32,
  sample_delta: u32
}

#[derive(Debug, Eq)]
pub struct STTSReader {
  data: Vec<u8>,
  size: u32,
  box_type: String,

  entry_count: Option<u32>,
  samples: Option<Vec<STTSSample>>
}

impl PartialEq for STTSReader {
  fn eq(&self, other: &Self) -> bool {
    self.size == other.size &&
    self.entry_count == other.entry_count
  }
}

impl STTSReader {

  pub fn parse(mp4: &[u8]) -> Result<STTSReader, CustomError> {
    let tkhd_option = find_box("moov", 0, mp4)
      .and_then(|moov|find_box("trak", 8, moov))
      .and_then(|trak|find_box("mdia", 8, trak))
      .and_then(|mdia|find_box("minf", 8, mdia))
      .and_then(|minf|find_box("stbl", 8, minf))
      .and_then(|stbl|find_box("stts", 8, stbl));
    
    if let Some(stts_data) = tkhd_option {
      Ok(STTSReader::get_reader(stts_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn get_reader(stts_data: &[u8]) -> Result<STTSReader, CustomError> {
    let mut start = 0usize;
    // Parse size
    let size = util::get_u32(stts_data, start)?;

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(stts_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!("{}", err),
    };

    Ok(STTSReader{
      data: stts_data.to_vec(),
      size,
      box_type,
      entry_count: Option::None,
      samples: Option::None
    })
  }

  pub fn get_entry_count(&mut self) -> Result<u32, CustomError> {
    if self.entry_count.is_none() {
      self.entry_count = Some(util::get_u32(&self.data, 12)?)
    }
    self.entry_count.ok_or(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::PARSE_BOX_ERROR),
        format!("{}: Unable to get entry count", CLASS),
        file!(),
        line!()))
  }

  pub fn get_samples(&mut self) {
    todo!()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_stts() {

    let stts: [u8; 16] = [
      // Size
      0x00, 0x00, 0x00, 0x10,
      // stts
      0x73, 0x74, 0x74, 0x73,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
  
    let expected_stts: STTSReader = STTSReader {
      data: (&stts).to_vec(),
      box_type: "stts".to_string(),
      size: 16,
      entry_count: Option::None,
      samples: Option::None
    };

    let mut stts_reader = STTSReader::get_reader(&stts).unwrap();
    assert_eq!(stts_reader, expected_stts);
    assert_eq!(stts_reader.get_entry_count().unwrap(), 0);
  }
}