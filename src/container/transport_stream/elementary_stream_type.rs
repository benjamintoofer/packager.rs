use std::fmt::Display;


/*
https://en.wikipedia.org/wiki/Program-specific_information#Elementary_stream_types
*/
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ElementaryStreamType {
  AAC,
  H_264,
  UNKNOWN
}

impl ElementaryStreamType {
  pub fn get_type(value: u8) -> ElementaryStreamType {
    match value {
      0xF => {ElementaryStreamType::AAC}
      0x1B => {ElementaryStreamType::H_264}
      _ => {ElementaryStreamType::UNKNOWN}
    }
  }

  pub fn get_value(&self) -> u8 {
    match self {
        ElementaryStreamType::AAC => {0xF}
        ElementaryStreamType::H_264 => {0x1B}
        ElementaryStreamType::UNKNOWN => {0x0}
    }
  }

  pub fn get_description(&self) -> String {
    match self {
        ElementaryStreamType::AAC => {"ISO/IEC 13818-7 ADTS AAC (MPEG-2 lower bit-rate audio)".to_string()}
        ElementaryStreamType::H_264 => {"ITU-T Rec. H.264 and ISO/IEC 14496-10 (lower bit-rate video)".to_string()}
        ElementaryStreamType::UNKNOWN => {"Uknown type".to_string()}
    }
  }
}

impl Display for ElementaryStreamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stream Type: {}; descritpion: {}", self.get_value(), self.get_description())
    }
}
