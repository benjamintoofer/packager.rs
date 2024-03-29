use crate::error::CustomError;
use crate::util;

static CLASS: &str = "AACAudioSpecificConfig";
#[derive(Debug, PartialEq, Eq)]
pub struct AACAudioSpecificConfig {
  pub audio_object_type: u8,                // 5 bit
  pub sampling_frequency_index: u8,         // 4 bit
  pub sampling_frequency: Option<u32>,      // 24 bit
  pub channel_configuration: u8,
}

impl AACAudioSpecificConfig {
  pub fn parse(data: &[u8]) -> Result<AACAudioSpecificConfig, CustomError> {
    let offset = 2usize;
    let temp = util::get_u8(data, offset)?;
    let audio_object_type = AACAudioSpecificConfig::get_audio_object_type(temp);

    // Assuming audio object is only 5 bits for now
    let temp_16 = util::get_u16(data, offset)?;
    let sampling_frequency_index = ((temp_16 & 0x780) >> 7) as u8;
    let sampling_frequency: Option<u32> = Option::None;
    if sampling_frequency_index == 0xf {
      // TODO (benjamintoofer@gmail.com): Parse sampling_frequency
      println!("AACAudioSpecificConfig: ERROR! Implement parsing sampling_frequency!");
    }
    let channel_configuration = ((temp_16 & 0x78) >> 3) as u8;
    Ok(AACAudioSpecificConfig {
      audio_object_type,
      sampling_frequency_index,
      sampling_frequency,
      channel_configuration,
    })
  }

  // 14496-3; 1.6.2.1 AudioSpecificConfig
  // Table 1.14 — Syntax of GetAudioObjectType()
  fn get_audio_object_type(data: u8) -> u8 {
    // 5 bit
    let audio_object_type: u8 = (data & 0xF8) >> 3;
    if audio_object_type == 31 {
      // audio_object_type = 32 + audio_object_type_ext;
    }
    audio_object_type
  }
}

pub struct AACAudioSpecificConfigBuilder {
  channel_count: u32,
  sampling_frequency_index: u32
}

impl AACAudioSpecificConfigBuilder {

  pub fn create_builder() -> AACAudioSpecificConfigBuilder {
    return AACAudioSpecificConfigBuilder {
      channel_count: 0,
      sampling_frequency_index: 0,
    }
  }

  pub fn channel_count(mut self, channel_count: u32) -> AACAudioSpecificConfigBuilder {
    self.channel_count = channel_count;
    self
  }

  pub fn sampling_frequency_index(mut self, sampling_frequency_index: u32) -> AACAudioSpecificConfigBuilder {
    self.sampling_frequency_index = sampling_frequency_index;
    self
  }
  /**
    audioObjectType = 2;          5 bits
    samplingFrequencyIndex;       4 bits
    channelConfiguration (1 | 2)  4 bits
    GASpecificConfig              3 bits
  */

  pub fn build(&self) -> Vec<u8> {
    let sample_and_channel:u32 = (self.sampling_frequency_index << 4) | self.channel_count;
    let mut result = 0x1000u32;
    result = result | (sample_and_channel << 3);
    let value = util::transform_u32_to_u8_array(result);
    vec![
      // DecSpecificInfoTag
      0x05,
      // length
      0x80, 0x80, 0x80, 0x02,
      value[1], value[0],
    ]
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_audio_specific_config() {
    let audio_specific_config: [u8; 4] = [
      0x05, 0x02, 0x11, 0x90
    ];
  
    let expected_config = AACAudioSpecificConfig{
      audio_object_type: 2,
      sampling_frequency_index: 3,
      sampling_frequency: Option::None,
      channel_configuration: 2,
    };
    assert_eq!(AACAudioSpecificConfig::parse(&audio_specific_config).unwrap(), expected_config);
  }

  #[test]
  fn test_audio_specific_config_builder() {
    let expected_audio_specific_config: [u8; 7] = [
      0x05,
      0x80, 0x80, 0x80, 0x02,
      0x12, 0x30
    ]; 
    let actual_audio_specific_config = AACAudioSpecificConfigBuilder::create_builder()
      .channel_count(6)
      .sampling_frequency_index(4)
      .build();
    assert_eq!(actual_audio_specific_config, expected_audio_specific_config);
  }
}
