use crate::error::CustomError;
use crate::container::isobmff::BoxBuilder;
use crate::container::isobmff::boxes::mdhd::MDHDBuilder;
use crate::container::isobmff::boxes::hdlr::HDLRBuilder;
use crate::container::isobmff::boxes::minf::MINFBuilder;
use crate::container::remux;
use crate::util;

// MediaBox 14496-12; 8.4.1

pub struct MDIABuilder {
  mdhd_builder: Option<MDHDBuilder>,
  hdlr_builder: Option<HDLRBuilder>,
  minf_builder: Option<MINFBuilder>,
}

impl MDIABuilder {
  pub fn create_builder() -> MDIABuilder {
    MDIABuilder{
      mdhd_builder: None,
      hdlr_builder: None,
      minf_builder: None,
    }
  }

  pub fn mdhd(mut self, mdhd_builder: MDHDBuilder) -> MDIABuilder {
    self.mdhd_builder = Some(mdhd_builder);
    self
  }

  pub fn hdlr(mut self, hdlr_builder: HDLRBuilder) -> MDIABuilder {
    self.hdlr_builder = Some(hdlr_builder);
    self
  }

  pub fn minf(mut self, minf_builder: MINFBuilder) -> MDIABuilder {
    self.minf_builder = Some(minf_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>,CustomError> {
    let mdhd = self.mdhd_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing mdhd_builder for MDIABuilder")))?
      .build()?;

    let hdlr = self.hdlr_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing hdlr_builder for MDIABuilder")))?
      .build();

    let minf = self.minf_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing minf_builder for MDIABuilder")))?
      .build()?;
    
    let size = 
      8 + // header
      mdhd.len() +
      hdlr.len() +
      minf.len();
    let size_array = util::transform_usize_to_u8_array(size);

    Ok(
      [
        vec![
          // Size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // mdia
          0x6D, 0x64, 0x69, 0x61,
        ],
        mdhd,
        hdlr,
        minf,
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_mdia() {
    let expected_mdia: [u8; 16] = [
      // Size
      0x00, 0x00, 0x00, 0x10,
      // mdia
      0x73, 0x74, 0x63, 0x6F,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let mdia = MDIABuilder::create_builder().build();
    assert_eq!(mdia, expected_mdia);
  }
}