
pub trait MinorError {
  fn message(&self) -> String;
  fn code(&self) -> u8;
}

pub enum MajorCode {
  ISOBMFF           = 0,
  TRANSPORT_STREAM  = 1,
  MANIFEST          = 2,
  UTIL              = 3,
  
}

#[allow(non_camel_case_types)]
pub enum ISOBMFFMinorCode {
  UNABLE_TO_FIND_BOX_ERROR  = 0,
  PARSE_BOX_ERROR           = 1,
}

#[allow(non_camel_case_types)]
pub enum TransportStreamMinorCode {
  PARSE_TS_ERROR  = 0,
}

#[allow(non_camel_case_types)]
pub enum ManifestMinorCode {
  // TODO
}

#[allow(non_camel_case_types)]
pub enum UtilMinorCode {
  PARSING_UNSIGNED_ERROR  = 0
}

impl MinorError for ISOBMFFMinorCode {
  fn message(&self) -> String {
      match self {
          ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR => { "Unable to find box".to_string()}
          ISOBMFFMinorCode::PARSE_BOX_ERROR => {"Error parsing isobmff box".to_string()}
      }
  }

  fn code(&self) -> u8 {
      match self {
          ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR => { ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR as u8 }
          ISOBMFFMinorCode::PARSE_BOX_ERROR => { ISOBMFFMinorCode:: PARSE_BOX_ERROR as u8 }
      }
  }
}

impl MinorError for ManifestMinorCode {
    fn message(&self) -> String {
        todo!()
    }

    fn code(&self) -> u8 {
        todo!()
    }
}

impl MinorError for UtilMinorCode {
    fn message(&self) -> String {
        match self {
            UtilMinorCode::PARSING_UNSIGNED_ERROR => { "An error occurred attempting to parse and unsigned value".to_string() }
        }
    }

    fn code(&self) -> u8 {
        match self {
            UtilMinorCode::PARSING_UNSIGNED_ERROR => { UtilMinorCode::PARSING_UNSIGNED_ERROR as u8 }
        }
    }
}