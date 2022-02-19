use crate::util;
use crate::error::CustomError;
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
    let mut index = 0usize;
    for nal_unit in self.nal_units {
      let nal_size: u32 = nal_unit.nal_unit.len() as u32;
      let nal_size_array = util::transform_u32_to_u8_array(nal_size).to_vec();
      let data = [
        vec![nal_size_array[3],nal_size_array[2],nal_size_array[1],nal_size_array[0]],
        nal_unit.nal_unit
      ].concat();
      let start = index;
      let end = start + data.len();
      nal_stream.splice(start..end, data.into_iter());
      index = end;
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
    let expected_mdat: [u8; 29] = [
      // size
      0x00, 0x00, 0x00, 0x1D,
      // mdat
      0x6D, 0x64, 0x61, 0x74,
      0x00, 0x00, 0x00, 0x03,
      0x00, 0x01, 0x02,
      0x00, 0x00, 0x00, 0x02,
      0x03, 0x04,
      0x00, 0x00, 0x00, 0x04,
      0x05, 0x06, 0x07, 0x08,
    ];

    let nal_units = vec![
      NalRep{
        pts: 1,
        dts: 1,
        nal_unit: vec![0x00,0x01,0x02]
      },
      NalRep{
        pts: 2,
        dts: 2,
        nal_unit: vec![0x03,0x04]
      },
      NalRep{
        pts: 3,
        dts: 3,
        nal_unit: vec![0x05,0x06,0x07,0x08]
      }
    ];
    
    let mdat = MDATBuilder::create_builder()
      .nal_units(nal_units)
      .build()
      .unwrap();
    assert_eq!(mdat, expected_mdat);
  }
}