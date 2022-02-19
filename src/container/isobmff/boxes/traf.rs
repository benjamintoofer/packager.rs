use crate::container::isobmff::boxes::{tfhd::TFHDBuilder, tfdt::TFDTBuilder, trun::TRUNBuilder};
use crate::container::remux;
use crate::error::CustomError;
use crate::util;

pub struct TRAFBuilder {
  tfhd_builder: Option<TFHDBuilder>,
  tfdt_builder: Option<TFDTBuilder>,
  trun_builder: Option<TRUNBuilder>,
  data_offset: usize,
}

impl TRAFBuilder {
  pub fn create_builder() -> TRAFBuilder {
    TRAFBuilder{
      tfhd_builder: None,
      tfdt_builder: None,
      trun_builder: None,
      data_offset: 0,
    }
  }

  pub fn tfhd(mut self, tfhd_builder: TFHDBuilder) -> TRAFBuilder {
    self.tfhd_builder = Some(tfhd_builder);
    self
  }

  pub fn tfdt(mut self, tfdt_builder: TFDTBuilder) -> TRAFBuilder {
    self.tfdt_builder = Some(tfdt_builder);
    self
  }

  pub fn trun(mut self, trun_builder: TRUNBuilder) -> TRAFBuilder {
    self.trun_builder = Some(trun_builder);
    self
  }

  pub fn set_data_offset(mut self, data_offset: usize) -> TRAFBuilder {
    self.data_offset = data_offset;
    self
  }

  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    let tfhd = self.tfhd_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing tfhd_builder for STBLBuilder")))?
      .build();
    let tfdt = self.tfdt_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing tfdt_builder for STBLBuilder")))?
      .build();
    let data_offset = self.data_offset + tfhd.len() + tfdt.len() + 8;
    let trun = self.trun_builder
      .ok_or_else(||remux::generate_error(String::from("Missing trun_builder for STBLBuilder")))?
      .data_offset(data_offset)
      .build();

    let size = 
      8 + // header
      tfhd.len() +
      tfdt.len() +
      trun.len();
    let size_array = util::transform_usize_to_u8_array(size);

    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // traf
          0x74, 0x72, 0x61, 0x66,
        ],
        tfhd,
        tfdt,
        trun
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::container::isobmff::nal::NalRep;

  #[test]
  fn test_build_traf() {
    let expected_traf: [u8; 92] = [
      // size
      0x00, 0x00, 0x00, 0x5C,
      // traf
      0x74, 0x72, 0x61, 0x66,
      // tfhd
      0x00, 0x00, 0x00, 0x1C,
      0x74, 0x66, 0x68, 0x64,
      0x00, 0x02, 0x00, 0x2A,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x0B, 0xB8,
      0x01, 0x01, 0x00, 0x00,
      // tfdt
      0x00, 0x00, 0x00, 0x14,
      0x74, 0x66, 0x64, 0x74,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x10, 0xA1, 0xD0,
      // trun 
      0x00, 0x00, 0x00, 0x24,
      0x74, 0x72, 0x75, 0x6E,
      0x00, 0x00, 0x02, 0x05,
      0x00, 0x00, 0x00, 0x03,
      0x00, 0x00, 0x00, 0x64,
      0x02, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x07,
      0x00, 0x00, 0x00, 0x06,
      0x00, 0x00, 0x00, 0x08,
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
    
    let traf = TRAFBuilder::create_builder()
      .tfhd(
        TFHDBuilder::create_builder()
          .track_id(1)
          .sample_duration(3000)
      )
      .tfdt(
        TFDTBuilder::create_builder()
          .base_media_decode_time(1090000)
      )
      .trun(
        TRUNBuilder::create_builder()
          .data_offset(100)
          .first_sample_flags(0x2000000)
          .flags(0x0205)
          .samples(nal_units)
      )
      .build()
      .unwrap();
    assert_eq!(traf, expected_traf);
  }
}