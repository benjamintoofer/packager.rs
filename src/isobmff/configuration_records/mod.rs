pub mod avcC;
pub mod es_descriptor;

enum DescriptorTags {
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