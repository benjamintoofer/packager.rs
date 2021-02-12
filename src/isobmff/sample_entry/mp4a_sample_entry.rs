use super::{sample_entry::{SampleEntry}};
use super::audio_sample_entry::AudioSampleEntry;
use crate::isobmff::descriptors::es_descriptor::ESDescriptor;
use crate::isobmff::boxes::iso_box::find_box;
use crate::isobmff::descriptors::find_descriptor;
use crate::isobmff::descriptors::DescriptorTags;

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
    let es_descriptor = find_box("esds", offset, data)
      .and_then(|esds_data| find_descriptor(DescriptorTags::ES_DESC, 12, esds_data)) 
      .map(|es_data| ESDescriptor::parse(es_data))
      .expect("Cannot parse ESDescriptor");

      // TODO (benjamintoofer@gmail.com): Add proper error handling around this.

    MP4ASampleEntry {
      sample_entry,
      audio_sample_entry,
      es_descriptor
    }
  }
}