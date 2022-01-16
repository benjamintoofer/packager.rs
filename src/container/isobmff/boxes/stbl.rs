use crate::util;
use crate::error::CustomError;
use crate::container::remux;
use crate::container::isobmff::boxes::stsd::STSDBuilder;
use crate::container::isobmff::boxes::stts::STTSBuilder;
use crate::container::isobmff::boxes::stsc::STSCBuilder;
use crate::container::isobmff::boxes::stsz::STSZBuilder;
use crate::container::isobmff::boxes::stco::STCOBuilder;

// SampleTableBox 14496-12; 8.5.1

pub struct STBLBuilder {
  stsd_builder: Option<STSDBuilder>
}

impl STBLBuilder {
  pub fn create_builder() -> STBLBuilder {
    STBLBuilder{
      stsd_builder: None
    }
  }

  pub fn stsd(mut self, stsd_builder: STSDBuilder) -> STBLBuilder {
    self.stsd_builder = Some(stsd_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, CustomError> {
    let stsd = self.stsd_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing stsd_builder for STBLBuilder")))?
      .build()?;
    let stts = STTSBuilder::create_builder().build();
    let stsc = STSCBuilder::create_builder().build();
    let stsz = STSZBuilder::create_builder().build();
    let stco = STCOBuilder::create_builder().build();

     let size = 
      8 + // header
      stsd.len() +
      stts.len() +
      stsc.len() + 
      stsz.len() + 
      stco.len();
    let size_array = util::transform_usize_to_u8_array(size);
    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // stbl
          0x73, 0x74, 0x62, 0x6C,
        ],
        stsd,
        stts,
        stsc,
        stsz,
        stco,
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::container::isobmff::BoxBuilder;

  struct MockHandler {}

  impl BoxBuilder for MockHandler {
    fn build(&self) -> Result<Vec<u8>, CustomError> {
      Ok(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
    }
  }

  #[test]
  fn test_build_stbl() {
    let expected_stbl: [u8; 100] = [
      // size
      0x00, 0x00, 0x00, 0x64,
      // stbl
      0x73, 0x74, 0x62, 0x6C,
      // stsd
      0x00, 0x00, 0x00, 0x18,
      0x73, 0x74, 0x73, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
      // stts
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x74, 0x74, 0x73,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // stsc
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x74, 0x73, 0x63,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // stsz
      0x00, 0x00, 0x00, 0x14,
      0x73, 0x74, 0x73, 0x7A,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // stco
      0x00, 0x00, 0x00, 0x10,
      0x73, 0x74, 0x63, 0x6F,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    let handler = Box::new(MockHandler{});
    let stsd_builder = STSDBuilder::create_builder()
      .sample_entry(handler);
    let stbl = STBLBuilder::create_builder()
      .stsd(stsd_builder)
      .build()
      .unwrap();
    assert_eq!(stbl, expected_stbl);
  }
}