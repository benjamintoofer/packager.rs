

// ChunkOffsetBox 14496-12; 8.7.5

#[derive(Debug, PartialEq, Eq)]
pub struct STCOBuilder {}

impl STCOBuilder {
  pub fn create_builder() -> STCOBuilder {
    STCOBuilder{}
  }

  pub fn build(&self) -> Vec<u8> {
    vec![
      // Size
      0x00, 0x00, 0x00, 0x10,
      // stco
      0x73, 0x74, 0x63, 0x6F,
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
  fn test_build_stco() {
    let expected_stco: [u8; 16] = [
      // Size
      0x00, 0x00, 0x00, 0x10,
      // stco
      0x73, 0x74, 0x63, 0x6F,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let stco = STCOBuilder::create_builder().build();
    assert_eq!(stco, expected_stco);
  }
}