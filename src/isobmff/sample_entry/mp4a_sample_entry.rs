use super::{audio_sample_entry, sample_entry::{self, SampleEntry}};
use super::audio_sample_entry::AudioSampleEntry;
use crate::isobmff::configuration_records::es_descriptor::ESDescriptor;
use crate::isobmff::boxes::iso_box::find_box;

#[derive(Debug)]
pub struct MP4ASampleEntry {
  pub sample_entry: SampleEntry,
  pub audio_sample_entry: AudioSampleEntry,
  pub es_descriptor: ESDescriptor
}

impl MP4ASampleEntry {
  pub fn parse(data: &[u8]) -> MP4ASampleEntry {
    let sample_entry = SampleEntry::parse(data);
    let (audio_sample_entry, offset) = AudioSampleEntry::parse(data);
    println!("----------------HELLO----------------------");
    println!("{:?}", sample_entry);
    println!("{:?}", audio_sample_entry);
    let es_descriptor = find_box("esds", offset, data)
      .map(|esds_data| ESDescriptor::parse(esds_data))
      // TODO (benjamintoofer@gmail.com): Add proper error handling around this.
      .unwrap()
      .expect("something");
    println!("------WHAT UP----");

    MP4ASampleEntry {
      sample_entry,
      audio_sample_entry,
      es_descriptor
    }
  }
}