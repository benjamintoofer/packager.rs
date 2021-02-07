use super::sample_entry::SampleEntry;
use super::visual_sample_entry:: VisualSampleEntry;
use crate::isobmff::configuration_records::avcC::AVCDecoderConfigurationRecord;
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
    let config = AVCDecoderConfigurationRecord::parse(data, offset);

    AVCSampleEntry {
      sample_entry,
      visual_sample_entry,
      config
    }
  }
}