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

  pub fn traf(mut self, traf_builder: TRAFBuilder) -> MOOFBuilder {
    self.traf_builder = Some(traf_builder);
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
  use crate::container::isobmff::boxes::tfhd::TFHDBuilder;
  use crate::container::isobmff::boxes::tfdt::TFDTBuilder;
  use crate::container::isobmff::boxes::trun::TRUNBuilder;
  use crate::container::writer::mp4_writer::SampleInfo;

  #[test]
  fn test_build_moof() {
    let expected_moof: [u8; 116] = [
      // size
      0x00, 0x00, 0x00, 0x74,
      // moof
      0x6D, 0x6F, 0x6F, 0x66,
      // mfhd
      0x00, 0x00, 0x00, 0x10,
      0x6D, 0x66, 0x68, 0x64,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // traf
      0x00, 0x00, 0x00, 0x5C,
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
      0x00, 0x00, 0x00, 0x7C,
      0x02, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x07,
      0x00, 0x00, 0x00, 0x06,
      0x00, 0x00, 0x00, 0x08,
    ];

    let samples = vec![
      SampleInfo{
        pts: 1,
        dts: 1,
        data: vec![0x00,0x01,0x02]
      },
      SampleInfo{
        pts: 2,
        dts: 2,
        data: vec![0x03,0x04]
      },
      SampleInfo{
        pts: 3,
        dts: 3,
        data: vec![0x05,0x06,0x07,0x08]
      }
    ];
    
    let moof = MOOFBuilder::create_builder()
      .traf(
        TRAFBuilder::create_builder()
          .tfhd(
            TFHDBuilder::create_builder()
              .sample_duration(3000)
              .track_id(1)
          )
          .tfdt(
            TFDTBuilder::create_builder()
              .base_media_decode_time(1090000)
          )
          .trun(
            TRUNBuilder::create_builder()
              .flags(0x0205)
              .first_sample_flags(0x2000000)
              .data_offset(100)
              .samples(samples)
          )
      )
      .build()
      .unwrap();
    assert_eq!(moof, expected_moof);
  }
}