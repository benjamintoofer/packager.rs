use crate::util;

static CLASS: &str = "AudioSampleEntry";
#[derive(Debug)]
pub struct AudioSampleEntry {
  channel_count: u16,
  sample_size: u16,
  sample_rate: u32,
}

impl AudioSampleEntry {
  pub fn parse(data: &[u8]) -> (AudioSampleEntry, usize) {
    let offset = 24usize;
    let mut start = offset;
    let channel_count = util::get_u16(data, start)
      .expect(format!("{}.parse.channel_count: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 2;
    // Parse sample size
    let sample_size = util::get_u16(data, start)
      .expect(format!("{}.parse.sample_size: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 6;
    // Parse sample size
    let sample_rate = util::get_u32(data, start)
      .and_then(|val| Ok(val >> 16u32))
      .expect(format!("{}.parse.sample_rate: cannot get u32 from start = {}",CLASS, start).as_ref());


    (AudioSampleEntry {
      channel_count,
      sample_size,
      sample_rate
    }, start + 4)
  }
}

#[derive(Debug)]
pub struct AudioSampleEntryBuilder {
  channel_count: u32,
  sample_rate: u32,
}

impl AudioSampleEntryBuilder {

  pub fn create_builder() -> AudioSampleEntryBuilder {
    return AudioSampleEntryBuilder {
      channel_count: 0,
      sample_rate: 0,
    }
  }

  pub fn channel_count(mut self, channel_count: u32) -> AudioSampleEntryBuilder {
    self.channel_count = channel_count;
    self
  }

  pub fn sample_rate(mut self, sample_rate: u32) -> AudioSampleEntryBuilder {
    self.sample_rate = sample_rate;
    self
  }

  pub fn build(&self) -> Vec<u8> {
    let channel_count_array = util::transform_u32_to_u8_array(self.channel_count);
    let sample_rate_array = util::transform_u32_to_u8_array(self.sample_rate);
    vec![
      // int(32)[2] reserved
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      // channelcount
      channel_count_array[1], channel_count_array[0],
      // samplesize
      0x00, 0x10,
      // int(16) pre_defined
      0x00, 0x00,
      // int(16) reserved
      0x00, 0x00,
      // samplerate
      sample_rate_array[3], sample_rate_array[2], sample_rate_array[1], sample_rate_array[0]
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_audio_sample_entry() {
    let expected_audio_sample_entry: [u8; 20] = [
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x02,
      0x00, 0x10,
      0x00, 0x00,
      0x00, 0x00,
      0x00, 0x00, 0x5D, 0xC0
    ];
    let audio_sample_entry = AudioSampleEntryBuilder::create_builder()
      .channel_count(2)
      .sample_rate(24000)
      .build();

    assert_eq!(audio_sample_entry, expected_audio_sample_entry);
  }
}