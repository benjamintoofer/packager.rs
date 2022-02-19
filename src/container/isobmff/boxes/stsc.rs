

// SampleToChunkBox 14496-12; 8.7.4

#[derive(Debug, PartialEq, Eq)]
pub struct STSCBuilder {}

impl STSCBuilder {
  pub fn create_builder() -> STSCBuilder {
    STSCBuilder{}
  }

  pub fn build(&self) -> Vec<u8> {
    vec![
      // size
      0x00, 0x00, 0x00, 0x10,
      // stsc
      0x73, 0x74, 0x73, 0x63,
      // version
      0x00,
      // flag
      0x00, 0x00, 0x00,
      // entry_count
      0x00, 0x00, 0x00, 0x00,
    ]
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_stsc() {
    let expected_stsc: [u8; 16] = [
      // Size
      0x00, 0x00, 0x00, 0x10,
      // stsc
      0x73, 0x74, 0x73, 0x63,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let stsc = STSCBuilder::create_builder().build();
    assert_eq!(stsc, expected_stsc);
  }
}