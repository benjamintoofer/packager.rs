use super::{audio_sample_entry::AudioSampleEntryBuilder, sample_entry::{SampleEntry, SampleEntryBuilder}};
use super::audio_sample_entry::AudioSampleEntry;
use crate::container::isobmff::{BoxBuilder, descriptors::es_descriptor::{ESDescriptor, ESDescriptorBuidler}};
use crate::container::isobmff::boxes::iso_box::find_box;
use crate::container::isobmff::descriptors::find_descriptor;
use crate::container::isobmff::descriptors::DescriptorTags;

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

pub struct MP4ASampleEntryBuilder {
  pub sample_entry_builder: Option<SampleEntryBuilder>,
  audio_sample_entry_builder: Option<AudioSampleEntryBuilder>,
  es_descriptor_builder: Option<ESDescriptorBuidler>,
}

impl MP4ASampleEntryBuilder {
  pub fn create_builder() -> MP4ASampleEntryBuilder {
    return MP4ASampleEntryBuilder {
      sample_entry_builder: None,
      audio_sample_entry_builder: None,
      es_descriptor_builder: None,
    }
  }

  pub fn sample_entry(mut self, sample_entry_builder: SampleEntryBuilder) -> MP4ASampleEntryBuilder {
    self.sample_entry_builder = Some(sample_entry_builder);
    self
  }

  pub fn audio_sample_entry(mut self, audio_sample_entry_builder: AudioSampleEntryBuilder) -> MP4ASampleEntryBuilder {
    self.audio_sample_entry_builder = Some(audio_sample_entry_builder);
    self
  }

  pub fn esds(mut self, esds_builder: ESDescriptorBuidler) -> MP4ASampleEntryBuilder {
    self.es_descriptor_builder = Some(esds_builder);
    self
  }
}

impl BoxBuilder for MP4ASampleEntryBuilder {
    fn build(&self) -> Result<Vec<u8>, crate::error::CustomError> {
        todo!()
    }
}