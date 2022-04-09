use crate::{container::{transport_stream::adts::ADTSFrame, writer::mp4_writer::SampleInfo}, util};
use crate::error::CustomError;
use crate::container::isobmff::nal::NalRep;

// MediaDataBox 14496-12; 8.1.1

pub struct MDATBuilder {
  // nal_units: Vec<NalRep>
  media_data: Vec<u8>
}

impl MDATBuilder {
  pub fn create_builder() -> MDATBuilder {
    MDATBuilder{
      // nal_units: vec![],
      media_data: vec![],
    }
  }

  // pub fn nal_units(mut self, nal_units: Vec<NalRep>) -> MDATBuilder {
  //   self.nal_units = nal_units;
  //   self
  // }

  pub fn media_data(mut self, media_data: Vec<u8>) -> MDATBuilder {
    self.media_data = media_data;
    self
  }

  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    let size = 
      8 + // header
      self.media_data.len();
    let size_array = util::transform_usize_to_u8_array(size);
    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // mdat
          0x6D, 0x64, 0x61, 0x74,
        ],
        self.media_data,
      ].concat()
    )
  }

  pub fn merge_samples(samples: Vec<SampleInfo>) -> Vec<u8> {
    let total_sample_size: usize = samples
      .iter()
      .map(|sample|sample.data.len())
      .sum();
    let mut sample_stream: Vec<u8> = vec![0; total_sample_size];
    let mut index = 0usize;
    for sample in samples {
      let start = index;
      let end = start + sample.data.len();
      sample_stream.splice(start..end, sample.data.into_iter());
      index = end;
    }
    sample_stream
  }

  // pub fn convert_nal_units(nal_units: Vec<NalRep>) -> Vec<u8> {
  //   let all_nal_size: usize = nal_units
  //     .iter()
  //     .map(|nal_unit|nal_unit.nal_unit.len() + 4)
  //     .sum();
  //   let mut nal_stream: Vec<u8> = vec![0; all_nal_size];
  //   let mut index = 0usize;
  //   for nal_unit in nal_units {
  //     let nal_size: u32 = nal_unit.nal_unit.len() as u32;
  //     let nal_size_array = util::transform_u32_to_u8_array(nal_size).to_vec();
  //     let data = [
  //       vec![nal_size_array[3],nal_size_array[2],nal_size_array[1],nal_size_array[0]],
  //       nal_unit.nal_unit
  //     ].concat();
  //     let start = index;
  //     let end = start + data.len();
  //     nal_stream.splice(start..end, data.into_iter());
  //     index = end;
  //   }
  //   nal_stream
  // }

  pub fn convert_adts_frames(adts_frames: Vec<ADTSFrame>) -> Vec<u8> {
    let all_adts_size: usize = adts_frames
      .iter()
      .map(|frame|frame.data.len())
      .sum();

    let mut adts_stream: Vec<u8> = vec![0; all_adts_size];
    let mut index = 0usize;
    for frame in adts_frames {
      let start = index;
      let end = start + frame.data.len();
      adts_stream.splice(start..end, frame.data.into_iter());
      index = end;
    }
    adts_stream
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

    let samples = vec![
      SampleInfo{
        pts: 1,
        dts: 1,
        data: vec![
          // nal unit header
          0x00, 0x00, 0x00, 0x03,
          // media data
          0x00,0x01,0x02
        ]
      },
      SampleInfo{
        pts: 2,
        dts: 2,
        data: vec![
          // nal unit header
          0x00, 0x00, 0x00, 0x02,
          // media data
          0x03,0x04
        ]
      },
      SampleInfo{
        pts: 3,
        dts: 3,
        data: vec![
          // nal unit header
          0x00, 0x00, 0x00, 0x04,
          // media data
          0x05,0x06,0x07,0x08
        ]
      }
    ];
    
    let mdat = MDATBuilder::create_builder()
      .media_data(MDATBuilder::merge_samples(samples))
      .build()
      .unwrap();
    assert_eq!(mdat, expected_mdat);
  }
}