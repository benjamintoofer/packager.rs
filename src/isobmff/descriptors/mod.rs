use std::convert::TryInto;
use std::convert::TryFrom;
use std::str;

pub mod es_descriptor;
pub mod dec_config_descriptor;
pub mod aac_audio_specific_config;

pub enum DescriptorTags {
  FORBIDDEN,
  OBJ_DESC,
  INITIAL_OBJ_DESC,
  ES_DESC,
  DECODER_CONFIG_DESC,
  DEC_SPECIFIC_INFO, 
  // NOTE (benjamintoofer@gmail.com): There are more tags. Add if necessary.
}

impl DescriptorTags {
  pub fn value(&self) -> u8 {
    match self {
        DescriptorTags::FORBIDDEN => {0u8}
        DescriptorTags::OBJ_DESC => {1u8}
        DescriptorTags::INITIAL_OBJ_DESC => {2u8}
        DescriptorTags::ES_DESC => {3u8}
        DescriptorTags::DECODER_CONFIG_DESC => {4u8}
        DescriptorTags::DEC_SPECIFIC_INFO => {5u8}
    }
  }
}

pub fn find_descriptor<'a>(search_tag: DescriptorTags, offset: usize, current_box_data: &'a [u8]) -> Option<&'a [u8]> {
  let mut tag_index: usize = offset;
    
  while tag_index < current_box_data.len() {
    let length_index = tag_index + 1;
    let tag = current_box_data[tag_index];
    let length = current_box_data[length_index] as usize;

    
    if tag == search_tag.value() {
      return Some(current_box_data[tag_index..(length_index + length)].as_ref())
    }
    tag_index += length;
  }
  None
}