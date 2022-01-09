use super::sample_entry::SampleEntry;
use super::visual_sample_entry:: VisualSampleEntry;
use crate::container::isobmff::boxes::iso_box::find_box;
use crate::container::isobmff::configuration_records::avcC::AVCDecoderConfigurationRecord;
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