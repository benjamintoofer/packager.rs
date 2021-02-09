use crate::util;

use super::DescriptorTags;

static CLASS: &str = "ESDescriptor";
#[derive(Debug)]
pub struct ESDescriptor {
  id: u16,
  stream_dependence_flag: bool,
  url_flag: bool,
  ocr_stream_flag: bool,
  stream_priority: u8,      // 5 bit
  depends_on_es_id: Option<u16>,
  url_length: Option<u8>,
  url_string: Option<String> // ??
  // ...
}

impl ESDescriptor {
  pub fn parse(data: &[u8]) -> Result<ESDescriptor, &str> {
    let mut start = 12usize;
    let mut end = start + 1;
    // Parse descriptor tag
    let descriptor_tag = util::get_u8(data, start, end)
      .expect(format!("{}.parse.descriptor_tag: cannot get u8 from start = {}; end = {}",CLASS, start, end).as_ref());
    
    if descriptor_tag != DescriptorTags::ES_DESC.value() {
      return Err("Wrong");
    }

    Ok(ESDescriptor{})
  }
}