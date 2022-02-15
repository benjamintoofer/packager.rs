use crate::util;
use crate::error::CustomError;
use crate::container::remux;
use crate::container::isobmff::nal::NalRep;

// MediaDataBox 14496-12; 8.1.1

pub struct MDATBuilder {
  nal_units: Vec<NalRep>
}

impl MDATBuilder {
  pub fn create_builder() -> MDATBuilder {
    MDATBuilder{
      nal_units: vec![]
    }
  }

  pub fn nal_units(mut self, nal_units: Vec<NalRep>) -> MDATBuilder {
    self.nal_units = nal_units;
    self
  }

  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    let all_nal_size: usize = self.nal_units
      .iter()
      .map(|nal_unit|nal_unit.nal_unit.len() + 4)
      .sum();
    let mut nal_stream: Vec<u8> = vec![0; all_nal_size];
    let index = 0usize;
    for nal_unit in self.nal_units {
      let nal_size: u32 = nal_unit.nal_unit.len() as u32;
      let nal_size_array = util::transform_u32_to_u8_array(nal_size).to_vec();
      let data = [
        nal_size_array,
        nal_unit.nal_unit
      ].concat();
      let start = index;
      let end = start + data.len();
      nal_stream.splice(start..end, data.into_iter());
    }
    let size = 
      8 + // header
      all_nal_size;
    let size_array = util::transform_usize_to_u8_array(size);
    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // mdat
          0x6D, 0x64, 0x61, 0x74,
        ],
        nal_stream,
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_mdat() {
    let expected_mdat: [u8; 16] = [
      // size
      0x00, 0x00, 0x00, 0x10,
      // mdat
      0x6D, 0x64, 0x61, 0x74,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    
    let mdat = MDATBuilder::create_builder()
      .build()
      .unwrap();
    assert_eq!(mdat, expected_mdat);
  }
}