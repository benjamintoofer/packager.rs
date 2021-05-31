use crate::util;

static CLASS: &str = "AACAudioSpecificConfig";
#[derive(Debug)]
pub struct AACAudioSpecificConfig {
  pub audio_object_type: u8,        // 5 bit
  pub sampling_frequency_index: u8, // 4 bit
  pub sampling_frequency: Option<u32>,      // 24 bit
}

impl AACAudioSpecificConfig {
  pub fn parse(data: &[u8]) -> AACAudioSpecificConfig {
    let start = 2usize;
    let audio_object_type = AACAudioSpecificConfig::get_audio_object_type(
      util::get_u8(data, start)
      .expect(format!("{}.parse.audio_object_type: cannot get u8 from start = {}",CLASS, start).as_ref())
    );
    AACAudioSpecificConfig {
      audio_object_type,
    }
  }

  fn get_audio_object_type(data: u8) -> u8 {
    // 5 bit
    let audio_object_type: u8 = (data & 0xF8) >> 3;
    if audio_object_type == 31 {
      // audio_object_type = 32 + audio_object_type_ext;
    }
    audio_object_type
  }
  // GetAudioObjectType
}
