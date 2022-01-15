use crate::util;
use crate::container::isobmff::boxes::dref::DREFBuilder;

// DataInformationBox 14496-12; 8.7.1

#[derive(Debug, PartialEq, Eq)]
pub struct DINFBuilder {}

impl DINFBuilder {
  pub fn create_builder() -> DINFBuilder {
    DINFBuilder{}
  }

  pub fn build(&self) -> Vec<u8> {
    let dref = DREFBuilder::create_builder().build();
    let size = 
      8 + // header
      dref.len();
    let size_array = util::transform_usize_to_u8_array(size);
    [
      vec![
        // Size
        size_array[3], size_array[2], size_array[1], size_array[0],
        // dinf
        0x64, 0x69, 0x6E, 0x66,
      ],
      dref,
    ].concat()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_dinf() {
    let expected_dinf: [u8; 36] = [
      //dinf
      0x00, 0x00, 0x00, 0x24,
      0x64, 0x69, 0x6E, 0x66,
      // dref
      0x00, 0x00, 0x00, 0x1C,
      0x64, 0x72, 0x65, 0x66,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x0C,
      0x75, 0x72, 0x6C, 0x20,
      0x00, 0x00, 0x00, 0x01,
    ];
    let dinf = DINFBuilder::create_builder().build();
    assert_eq!(dinf, expected_dinf);
  }
}