use crate::container::isobmff::BoxBuilder;

// SoundMediaHeaderBox 14496-12; 8.4.5.3

#[derive(Debug, PartialEq, Eq)]
pub struct SMHDBuilder {}

impl SMHDBuilder {
  pub fn create_builder() -> SMHDBuilder {
    SMHDBuilder{}
  }
}

impl BoxBuilder for SMHDBuilder {
  fn build(&self) -> Result<Vec<u8>, crate::error::CustomError> {
    Ok(vec![
        // Size
        0x00, 0x00, 0x00, 0x10,
        // smhd
        0x73, 0x6d, 0x68, 0x64,
        // version
        0x00,
        // flag
        0x00, 0x00, 0x00,
        // balance
        0x00, 0x00,
        // reserved
        0x00, 0x00,
      ]
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_smhd() {
    let expected_smhd: [u8; 16] = [
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x6d, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    let smhd = SMHDBuilder::create_builder().build().unwrap();
    assert_eq!(smhd, expected_smhd);
  }
}