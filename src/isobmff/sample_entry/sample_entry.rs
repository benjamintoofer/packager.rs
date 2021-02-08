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
    let end = start + 2;
    let data_reference_index = util::get_u16(data, start, end)
      .expect(format!("{}.parse_sample_entry_data.data_reference_index: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());
    
    SampleEntry{ data_reference_index }
  }
}