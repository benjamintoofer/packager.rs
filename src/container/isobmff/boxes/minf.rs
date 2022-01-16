use crate::util;
use crate::container::remux;
use crate::container::isobmff::BoxBuilder;
use crate::container::isobmff::boxes::dinf::DINFBuilder;
use crate::container::isobmff::boxes::stbl::STBLBuilder;

// MediaInformationBox 14496-12; 8.4.4

pub struct MINFBuilder {
  media_header_builder: Option<Box<dyn BoxBuilder>>,
  stbl_builder: Option<STBLBuilder>
}

impl MINFBuilder {
  pub fn create_builder() -> MINFBuilder {
    MINFBuilder{
      media_header_builder: None,
      stbl_builder: None,
    }
  }

  pub fn media_header(mut self, media_header_builder: Box<dyn BoxBuilder>) -> MINFBuilder {
    self.media_header_builder = Some(media_header_builder);
    self
  }

  pub fn stbl(mut self, stbl_builder: STBLBuilder) -> MINFBuilder {
    self.stbl_builder = Some(stbl_builder);
    self
  }
}

impl BoxBuilder for MINFBuilder {
  fn build(&self) -> Result<Vec<u8>, crate::error::CustomError> {
    let media_header = self.media_header_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing media_header_builder for MINFBuilder")))?
      .build()?;
    let dinf = DINFBuilder::create_builder().build();
    let stbl = self.stbl_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing stbl_builder for MINFBuilder")))?
      .build()?;
    
    let size = 
      8 + // header
      media_header.len() +
      dinf.len() +
      stbl.len();
    let size_array = util::transform_usize_to_u8_array(size);

    Ok(
      [
        vec![
          // Size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // minf
          0x6D, 0x69, 0x6E, 0x66,
        ],
        media_header,
        dinf,
        stbl,
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::{container::isobmff::boxes::vmhd::VMHDBuilder, error::CustomError};
  use crate::container::isobmff::boxes::stbl::STBLBuilder;
  use crate::container::isobmff::boxes::stsd::STSDBuilder;

  struct MockHandler {}

  impl BoxBuilder for MockHandler {
    fn build(&self) -> Result<Vec<u8>, CustomError> {
      Ok(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
    }
  }

  #[test]
  fn test_build_minf() {
    let expected_minf: [u8; 164] = [
      // minf
      0x00, 0x00, 0x00, 0xA4,
      0x6D, 0x69, 0x6E, 0x66,
      // vmhd
      0x00, 0x00, 0x00, 0x14,
      0x76, 0x6D, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00 ,0x00, 0x00,
      //dinf
      0x00, 0x00, 0x00, 0x24,
      0x64, 0x69, 0x6E, 0x66,
      0x00, 0x00, 0x00, 0x1C,
      0x64, 0x72, 0x65, 0x66,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x0C,
      0x75, 0x72, 0x6C, 0x20,
      0x00, 0x00, 0x00, 0x01,
      // stbl
      0x00, 0x00, 0x00, 0x64,
      0x73, 0x74, 0x62, 0x6C,
      0x00, 0x00, 0x00, 0x18,
      0x73, 0x74, 0x73, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x74, 0x74, 0x73,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x74, 0x73, 0x63,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x14,
      0x73, 0x74, 0x73, 0x7A,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x74, 0x63, 0x6F,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    
    let minf = MINFBuilder::create_builder()
      .media_header(
        Box::new(VMHDBuilder::create_builder())
      )
      .stbl(
        STBLBuilder::create_builder()
          .stsd(
            STSDBuilder::create_builder()
            .sample_entry(Box::new(MockHandler{}))
          )
      )
      .build()
      .unwrap();
    assert_eq!(minf, expected_minf);
  }
}