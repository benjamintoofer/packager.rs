use crate::util;

static CLASS: &str = "AACAudioSpecificConfig";
#[derive(Debug)]
pub struct AACAudioSpecificConfig {
  audio_object_type: u8,      // 5 bit
}

impl AACAudioSpecificConfig {
  pub fn parse(data: &[u8]) -> AACAudioSpecificConfig {
    let start = 2usize;
    let end = start + 1;
    let audio_object_type = AACAudioSpecificConfig::get_audio_object_type(
      util::get_u8(data, start, end)
      .expect(format!("{}.parse.audio_object_type: cannot get u8 from start = {}; end = {}",CLASS, start, end).as_ref())
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
