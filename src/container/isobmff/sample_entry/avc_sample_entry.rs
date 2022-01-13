use crate::util;
use super::sample_entry::{SampleEntry, SampleEntryBuilder};
use super::visual_sample_entry:: {VisualSampleEntry, VisualSampleEntryBuilder};
use crate::{container::isobmff::boxes::iso_box::find_box, error::CustomError};
use crate::container::isobmff::configuration_records::avcC::{AVCDecoderConfigurationRecord, AVCDecoderConfigurationRecordBuilder};
use crate::error::{construct_error, error_code::{RemuxMinorCode, MajorCode}};
#[derive(Debug)]
pub struct AVCSampleEntry {
  pub sample_entry: SampleEntry,
  pub visual_sample_entry: VisualSampleEntry,
  pub config: AVCDecoderConfigurationRecord
}

impl AVCSampleEntry {
  pub fn parse(data: &[u8]) -> AVCSampleEntry {
    let sample_entry = SampleEntry::parse(data);
    let (visual_sample_entry, offset) = VisualSampleEntry::parse(data);
    #[allow(non_snake_case)]
    let avcC: AVCDecoderConfigurationRecord = find_box("avcC", offset, data)
      .map(|avcc_data| AVCDecoderConfigurationRecord::parse(avcc_data))
      // TODO (benjamintoofer@gmail.com): Add proper error handling around this.
      .unwrap();
      // .expect("No avcC box found in avc1");

    AVCSampleEntry {
      sample_entry,
      visual_sample_entry,
      config: avcC
    }
  }
}

pub struct AVCSampleEntryBuilder {
  sample_entry_builder: Option<SampleEntryBuilder>,
  visual_sample_entry_builder: Option<VisualSampleEntryBuilder>,
  avc_c_builder: Option<AVCDecoderConfigurationRecordBuilder>,
}

impl AVCSampleEntryBuilder {
  pub fn create_builder() -> AVCSampleEntryBuilder {
    return AVCSampleEntryBuilder {
      sample_entry_builder: None,
      visual_sample_entry_builder: None,
      avc_c_builder: None,
    }
  }

  pub fn sample_entry(mut self, sample_entry_builder: SampleEntryBuilder) -> AVCSampleEntryBuilder {
    self.sample_entry_builder = Some(sample_entry_builder);
    self
  }

  pub fn visual_sample_entry(mut self, visual_sample_entry_builder: VisualSampleEntryBuilder) -> AVCSampleEntryBuilder {
    self.visual_sample_entry_builder = Some(visual_sample_entry_builder);
    self
  }

  pub fn avc_c(mut self, avc_c_builder: AVCDecoderConfigurationRecordBuilder) -> AVCSampleEntryBuilder {
    self.avc_c_builder = Some(avc_c_builder);
    self
  }

  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    let sample_entry = self.sample_entry_builder
      .ok_or_else(||generate_error(String::from("Missing sample_entry_builder for AVCSampleEntryBuilder")))?
      .build();
    let visual_sample_entry = self.visual_sample_entry_builder
      .ok_or_else(||generate_error(String::from("Missing visual_sample_entry_builder for AVCSampleEntryBuilder")))?
      .build()?;
    let avc_c = self.avc_c_builder
      .ok_or_else(||generate_error(String::from("Missing avcC_builder for AVCSampleEntryBuilder")))?
      .build()?;
    let size = 
      8 + // header
      sample_entry.len() +
      visual_sample_entry.len() +
      avc_c.len();
    let size_array = util::transform_usize_to_u8_array(size);
    let avc1: Vec<u8> = [
      vec![
        // size
        size_array[3], size_array[2], size_array[1], size_array[0],
        // avc1
        0x61, 0x76, 0x63, 0x31,
      ],
      sample_entry,
      visual_sample_entry,
      avc_c,
    ].concat();

    Ok(avc1)
  }
}

fn generate_error(message: String) -> CustomError {
  return  construct_error(
    MajorCode::REMUX, 
    Box::new(RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR),
    message,
    file!(), 
    line!());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_avc1_sample_entry() {
    let expected_avc1_sample_entry = vec![
      0x00, 0x00, 0x00, 0x96, 0x61, 0x76, 0x63, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xE0, 0x01, 0x0E, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 0x54,
      0x6f, 0x6f, 0x66, 0x65, 0x72, 0x20, 0x41, 0x56, 0x43, 0x20, 0x43, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x30, 0x61, 0x76, 0x63, 0x43, 0x01, 0x42, 0xC0, 0x1E, 0xFF, 0xE1, 0x00, 0x19, 0x67, 0x42,
      0xC0, 0x1E, 0xD9, 0x01, 0xE0, 0x8F, 0xEB, 0x01, 0x10, 0x00, 0x00, 0x03, 0x00, 0x10, 0x00, 0x00, 0x03, 0x03, 0xC0, 0xF1, 0x62, 0xE4, 0x80, 0x01, 0x00, 0x04,
      0x68, 0xCB, 0x8C, 0xB2,
    ];
    let sps: [u8; 25] = [
      0x67, 0x42, 0xC0, 0x1E, 0xD9, 0x01, 0xE0, 0x8F, 0xEB, 0x01, 0x10, 0x00, 0x00, 0x03, 0x00, 0x10, 0x00, 0x00, 0x03, 0x03, 0xC0, 0xF1, 0x62, 0xE4, 0x80
    ];
    let pps: [u8; 4] = [
      0x68, 0xcb, 0x8c, 0xb2
    ];
    let avc1_sample_entry = AVCSampleEntryBuilder::create_builder()
      .sample_entry(
        SampleEntryBuilder::create_builder()
      )
      .visual_sample_entry(
        VisualSampleEntryBuilder::create_builder()
          .sps(&sps)
      )
      .avc_c(
        AVCDecoderConfigurationRecordBuilder::create_builder()
          .sps(&sps)
          .pps(&pps)
      )
      .build()
      .unwrap();

    assert_eq!(avc1_sample_entry, expected_avc1_sample_entry);
  }
}