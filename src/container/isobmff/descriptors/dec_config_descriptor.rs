use crate::util;
use crate::error::CustomError;
use crate::container::remux;
use super::{aac_audio_specific_config::AACAudioSpecificConfigBuilder, find_descriptor};
use super::DescriptorTags;
use super::aac_audio_specific_config::AACAudioSpecificConfig;

// 14496-1; 7.2.6.6
static CLASS: &str = "DecoderConfigDescriptor";
#[derive(Debug)]
pub struct DecoderConfigDescriptor {
  pub object_type_indication: u8,
  stream_type: u8,              // 6 bit
  upstream: bool,               // 1 bit
  buffer_size_db: u32,          // 24 bit
  max_bitrate: u32,
  avg_bitrate: u32,
  pub audio_sepcific_info: AACAudioSpecificConfig
}

impl  DecoderConfigDescriptor {
  pub fn parse(data: &[u8]) -> DecoderConfigDescriptor {
    let mut start = 2usize;
    // Parse object_type_indication
    let object_type_indication = util::get_u8(data, start)
      .expect(format!("{}.parse.object_type_indication: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    let temp = util::get_u8(data, start)
      .expect(format!("{}.parse.temp: cannot get u8 from start = {}",CLASS, start).as_ref());
    let stream_type = (temp & 0xFC) >> 2;
    let upstream = (temp & 0x2) != 0;

    let mut buffer_size_db: u32 = 0;
    for i in 0..3 {
      start = start +1;
      let buff = util::get_u8(data, start)
        .expect(format!("{}.parse.buff: cannot get u8 from start = {}",CLASS, start).as_ref());
        buffer_size_db =  buffer_size_db | (u32::from(buff) << (8 * (2 - i)));
    }

    start = start + 1;
    let max_bitrate = util::get_u32(data, start)
      .expect(format!("{}.parse.temp: cannot get u32 from start = {}",CLASS, start).as_ref());

    start = start + 4;
    let avg_bitrate = util::get_u32(data, start)
      .expect(format!("{}.parse.avg_bitrate: cannot get u32 from start = {}",CLASS, start).as_ref());

    let audio_sepcific_info = find_descriptor(DescriptorTags::DEC_SPECIFIC_INFO, start + 4, data)
      .and_then(|dec_info|Some(AACAudioSpecificConfig::parse(dec_info)))
      .expect("No DecoderConfigDescriptor")
      .unwrap();
    DecoderConfigDescriptor {
      object_type_indication,
      stream_type,
      upstream,
      buffer_size_db,
      max_bitrate,
      avg_bitrate,
      audio_sepcific_info
    }
  }
}

pub struct DecoderConfigDescriptorBuilder {
  aac_audio_specific_config_builder: Option<AACAudioSpecificConfigBuilder>
}

impl DecoderConfigDescriptorBuilder {
  pub fn create_builder() -> DecoderConfigDescriptorBuilder {
    return DecoderConfigDescriptorBuilder {
      aac_audio_specific_config_builder: None
    }
  }

  pub fn aac_audio_specific_config(mut self, aac_config_builder: AACAudioSpecificConfigBuilder) -> DecoderConfigDescriptorBuilder {
    self.aac_audio_specific_config_builder = Some(aac_config_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, CustomError> {
    let aac_audio_specific_config = self.aac_audio_specific_config_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing aac_audio_specific_config for DecoderConfigDescriptorBuilder")))?
      .build();

    let length: u8 = 13 + aac_audio_specific_config.len() as u8;
    Ok([
      vec![
        // DecoderConfigDescrTag
        0x04,
        // length
        0x80, 0x80, 0x80, length,
        // objectTypeIndication (0x40 == Audio ISO/IEC 14496-3 (AAC audio specific config))
        0x40,
        // bit(6) streamType (5 == audio stream); bit(1) upStream(0); const bit(1) reserved=1
        0x15,
        // bufferSizeDB
        0x00, 0x00, 0x00,
        // maxBitrate
        0x00, 0x00, 0x00, 0x00,
        // avgBitrate
        0x00, 0x00, 0x00, 0x00,
      ],
      aac_audio_specific_config
    ].concat())
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_audio_specific_config_builder() {
    let expected_decoder_config_descriptor: [u8; 25] = [
      // DecoderConfigDescriptor
      0x04,
      0x80, 0x80, 0x80, 0x14,
      0x40,
      0x15,
      0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x05,
      0x80, 0x80, 0x80, 0x02,
      0x12, 0x30
    ]; 
    let actual_decoder_config_descriptor = DecoderConfigDescriptorBuilder::create_builder()
      .aac_audio_specific_config(
        AACAudioSpecificConfigBuilder::create_builder()
          .channel_count(6)
          .sampling_frequency_index(4)
      )
      .build()
      .unwrap();
    assert_eq!(actual_decoder_config_descriptor, expected_decoder_config_descriptor);
  }
}