// MovieFragmentHeaderBox 14496-12; 8.8.5

pub struct MFHDBuilder {}

impl MFHDBuilder {
  pub fn create_builder() -> MFHDBuilder {
    MFHDBuilder{}
  }

  pub fn build(&self) -> Vec<u8> {
    vec![
      // size
      0x00, 0x00, 0x00, 0x10,
      // mfhd
      0x6D, 0x66, 0x68, 0x64,
      // version and flags
      0x00, 0x00, 0x00, 0x00,
      // sequence_number
      0x00, 0x00, 0x00, 0x00,
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_mfhd() {
    let expected_mfhd: [u8; 16] = [
      // size
      0x00, 0x00, 0x00, 0x10,
      // mfhd
      0x6D, 0x66, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    
    let mfhd = MFHDBuilder::create_builder()
      .build();
    assert_eq!(mfhd, expected_mfhd);
  }
}