pub mod hls_generator;
pub mod hls_writer;

pub enum HLSVersion {
  _4,
  _7,
  // TODO (benjamintoofer@gmail.com): Add version 8
}

impl HLSVersion {
  pub fn value(&self) -> u8 {
    match self {
        HLSVersion::_4 => {4u8}
        HLSVersion::_7 => {7u8}
    }
  }
}

#[allow(non_camel_case_types)]
pub enum HLSMediaType {
  AUDIO,
  VIDEO,
  CLOSED_CAPTIONS,
  SUBTITLES
}

impl HLSMediaType {
  pub fn value(&self) -> &str {
    match self {
        HLSMediaType::AUDIO => {"AUDIO"}
        HLSMediaType::VIDEO => {"VIDEO"}
        HLSMediaType::CLOSED_CAPTIONS => {"CLOSED-CAPTIONS"}
        HLSMediaType::SUBTITLES => {"SUBTITLES"}
    }
  }
}