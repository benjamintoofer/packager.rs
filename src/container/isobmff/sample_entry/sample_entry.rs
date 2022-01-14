use std::u16;

use crate::util;

static CLASS: &str = "SampleEntry";

#[derive(Debug)]
pub struct SampleEntry {
  data_reference_index: u16
}

impl SampleEntry {
  pub fn parse(data: &[u8]) -> SampleEntry {
    let start = 14usize;
    let data_reference_index = util::get_u16(data, start)
      .expect(format!("{}.parse_sample_entry_data.data_reference_index: cannot get u32 from start = {}",CLASS, start).as_ref());
    
    SampleEntry{ data_reference_index }
  }
}

pub struct SampleEntryBuilder {}

impl SampleEntryBuilder {
  pub fn create_builder() -> SampleEntryBuilder {
    return SampleEntryBuilder{}
  }
  pub fn build(&self) -> Vec<u8> {
    vec![
      // int(8)[6] reserved
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      // data_reference_index
      0x00, 0x01,
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_sample_entry() {
    let expected_sample_entry = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00,0x00, 0x01];
    let sample_entry = SampleEntryBuilder::create_builder().build();

    assert_eq!(sample_entry, expected_sample_entry);
  }
}
