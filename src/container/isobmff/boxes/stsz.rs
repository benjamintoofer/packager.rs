

// SampleSizeBox 14496-12; 8.7.3.2

#[derive(Debug, PartialEq, Eq)]
pub struct STSZBuilder {}

impl STSZBuilder {
  pub fn create_builder() -> STSZBuilder {
    STSZBuilder{}
  }

  pub fn build(&self) -> Vec<u8> {
    vec![
      // Size
      0x00, 0x00, 0x00, 0x14,
      // stsz
      0x73, 0x74, 0x73, 0x7A,
      // version
      0x00,
      // flag
      0x00, 0x00, 0x00,
      // sample_size
      0x00, 0x00, 0x00, 0x00,
      // sample_count
      0x00, 0x00, 0x00, 0x00,
    ]
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_stsz() {
    let expected_stsz: [u8; 20] = [
      0x00, 0x00, 0x00, 0x14,
      0x73, 0x74, 0x73, 0x7A,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    let stsz = STSZBuilder::create_builder().build();
    assert_eq!(stsz, expected_stsz);
  }
}