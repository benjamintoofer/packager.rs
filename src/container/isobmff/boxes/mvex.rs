
use crate::util;
use crate::container::remux;
use crate::error::CustomError;
use crate::container::isobmff::boxes::trex::TREXBuilder;

// MovieExtendsBox 14496-12; 8.8.1

pub struct MVEXBuilder {
  trex_builder: Option<TREXBuilder>,
}

impl MVEXBuilder {
  pub fn create_builder() -> MVEXBuilder {
    MVEXBuilder{
      trex_builder: None,
    }
  }

  pub fn trex(mut self, trex_builder: TREXBuilder) -> MVEXBuilder {
    self.trex_builder = Some(trex_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, CustomError> {
    let trex = self.trex_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing trex_builder for MVEXBuilder")))?
      .build();
   

    let size = 
      8 + // header
      trex.len();
    let size_array = util::transform_usize_to_u8_array(size);

    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // mvex
          0x6D, 0x76, 0x65, 0x78,
        ],
        trex,
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_mvex() {
    let expected_mvex: [u8; 40] = [
      // mvex
      0x00, 0x00, 0x00, 0x28,
      0x6D, 0x76, 0x65, 0x78,
      // trex
      0x00, 0x00, 0x00, 0x20,
      0x74, 0x72, 0x65, 0x78,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x02,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    let mvex = MVEXBuilder::create_builder()
      .trex(
        TREXBuilder::create_builder()
        .track_id(2)
      )
      .build()
      .unwrap();
    assert_eq!(mvex, expected_mvex);
  }
}