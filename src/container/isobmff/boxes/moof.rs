use crate::util;
use crate::error::CustomError;
use crate::container::remux;
use crate::container::isobmff::boxes::mfhd::MFHDBuilder;
use crate::container::isobmff::boxes::traf::TRAFBuilder;

// MovieFragmentBox 14496-12; 8.8.4

pub struct MOOFBuilder {
  traf_builder: Option<TRAFBuilder>
}

impl MOOFBuilder {
  pub fn create_builder() -> MOOFBuilder {
    MOOFBuilder{
      traf_builder: None
    }
  }

  pub fn traf(mut self, trac_builder: TRAFBuilder) -> MOOFBuilder {
    self.traf_builder = Some(trac_builder);
    self
  }

  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    let mfhd = MFHDBuilder::create_builder().build();
    let data_offset = 8 + mfhd.len();
    let traf = self.traf_builder
      .ok_or_else(||remux::generate_error(String::from("Missing traf_builder for MOOFBuilder")))?
      .set_data_offset(data_offset)
      .build()?;
    let size = 
      8 + // header
      mfhd.len() +
      traf.len();
    let size_array = util::transform_usize_to_u8_array(size);
    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // moof
          0x6D, 0x6F, 0x6F, 0x66,
        ],
        mfhd,
        traf,
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_moof() {
    let expected_moof: [u8; 16] = [
      // size
      0x00, 0x00, 0x00, 0x10,
      // moof
      0x6D, 0x66, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    
    let moof = MOOFBuilder::create_builder()
      .build()
      .unwrap();
    assert_eq!(moof, expected_moof);
  }
}