use crate::container::isobmff::BoxBuilder;

// VideoMediaHeaderBox 14496-12; 8.4.5.2

#[derive(Debug, PartialEq, Eq)]
pub struct VMHDBuilder {}

impl VMHDBuilder {
  pub fn create_builder() -> VMHDBuilder {
    VMHDBuilder{}
  }
}

impl BoxBuilder for VMHDBuilder {
  fn build(&self) -> Result<Vec<u8>, crate::error::CustomError> {
    Ok(vec![
        // Size
        0x00, 0x00, 0x00, 0x14,
        // vmhd
        0x76, 0x6D, 0x68, 0x64,
        // version
        0x00,
        // flag
        0x00, 0x00, 0x01,
        // graphicsmode
        0x00, 0x00,
        // int(16)[3] opcolor
        0x00, 0x00, 0x00, 0x00,0x00, 0x00,
      ]
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_vmhd() {
    let expected_vmhd: [u8; 20] = [
      0x00, 0x00, 0x00, 0x14,
      0x76, 0x6D, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00 ,0x00, 0x00,
    ];
    let vmhd = VMHDBuilder::create_builder().build().unwrap();
    assert_eq!(vmhd, expected_vmhd);
  }
}