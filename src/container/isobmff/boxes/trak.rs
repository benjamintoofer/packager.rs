use crate::error::CustomError;
use crate::container::remux;
use crate::util;
use crate::container::isobmff::boxes::tkhd::TKHDBuilder;
use crate::container::isobmff::boxes::mdia::MDIABuilder;

// TrackBox 14496-12; 8.3.1

pub struct TRAKBuilder {
  tkhd_builder: Option<TKHDBuilder>,
  mdia_builder: Option<MDIABuilder>
}

impl TRAKBuilder {
  pub fn create_builder() -> TRAKBuilder {
    TRAKBuilder{
      tkhd_builder: None,
      mdia_builder: None,
    }
  }

  pub fn tkhd(mut self, tkhd_builder: TKHDBuilder) -> TRAKBuilder {
    self.tkhd_builder = Some(tkhd_builder);
    self
  }

  pub fn mdia(mut self, mdia_builder: MDIABuilder) -> TRAKBuilder {
    self.mdia_builder = Some(mdia_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, CustomError> {
    let tkhd = self.tkhd_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing tkhd_builder for TRAKBuilder")))?
      .build();
    let mdia = self.mdia_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing mdia_builder for TRAKBuilder")))?
      .build()?;
    let size = 
      8 + // header
      tkhd.len() +
      mdia.len();
    let size_array = util::transform_usize_to_u8_array(size);
    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // trak
          0x74, 0x72, 0x61, 0x6B,
        ],
        tkhd,
        mdia,
      ].concat()
    )
  }
}

// #[cfg(test)]
// mod tests {

//   use super::*;

//   #[test]
//   fn test_build_trak() {
//     let expected_trak: [u8; 16] = [
//       // Size
//       0x00, 0x00, 0x00, 0x10,
//       // trak
//       0x73, 0x74, 0x73, 0x7A,
//       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
//     ];
//     let trak = TRAKBuilder::create_builder().build();
//     assert_eq!(trak, expected_trak);
//   }
// }