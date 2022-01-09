
pub trait MinorError {
  fn message(&self) -> String;
  fn code(&self) -> u8;
}

pub enum MajorCode {
  ISOBMFF           = 0,
  TRANSPORT_STREAM  = 1,
  MANIFEST          = 2,
  UTIL              = 3,
  NAL               = 4,
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
  PARSING_UNSIGNED_ERROR    = 0,
  PARSING_BIT_READER_ERROR  = 1,
}

#[allow(non_camel_case_types)]
pub enum NalMinorCode {
  UNEXPTED_NAL_UNIT_LENGTH          = 0,
  BYTE_STREAM_MISSING_START_PREFIX  = 1,
  UKNOWN_NAL_UNIT_TYPE              = 2,
}

impl MinorError for ISOBMFFMinorCode {
  fn message(&self) -> String {
    match self {
      ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR => { "Unable to find box".to_string() }
      ISOBMFFMinorCode::PARSE_BOX_ERROR => {"Error parsing isobmff box".to_string() }
    }
  }

  fn code(&self) -> u8 {
    match self {
      ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR => { ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR as u8 }
      ISOBMFFMinorCode::PARSE_BOX_ERROR => { ISOBMFFMinorCode:: PARSE_BOX_ERROR as u8 }
    }
  }
}

impl MinorError for TransportStreamMinorCode {
    fn message(&self) -> String {
      match self {
          TransportStreamMinorCode::PARSE_TS_ERROR => { "Unable to parse transport stream".to_string() }
      }
    }

    fn code(&self) -> u8 {
      match self {
          TransportStreamMinorCode::PARSE_TS_ERROR => { TransportStreamMinorCode::PARSE_TS_ERROR as u8 }
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
      UtilMinorCode::PARSING_BIT_READER_ERROR => { "An error occurred attempting to parse with the bit reader".to_string() }
    }
  }

  fn code(&self) -> u8 {
    match self {
      UtilMinorCode::PARSING_UNSIGNED_ERROR => { UtilMinorCode::PARSING_UNSIGNED_ERROR as u8 }
      UtilMinorCode::PARSING_BIT_READER_ERROR => { UtilMinorCode::PARSING_BIT_READER_ERROR as u8 }
    }
  }
}
impl MinorError for NalMinorCode {
  fn message(&self) -> String {
    match self {
      NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH => { "Unexpected NAL unit length".to_string()}
      NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX => { "Byte stream is missing starting prefix of 0x00000001".to_string() }
      NalMinorCode::UKNOWN_NAL_UNIT_TYPE => { "Uknown NAL Unit type".to_string() }
    }
  }

  fn code(&self) -> u8 {
    match self {
      NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH => { NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH  as u8 }
      NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX => { NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX as u8 }
      NalMinorCode::UKNOWN_NAL_UNIT_TYPE => { NalMinorCode::UKNOWN_NAL_UNIT_TYPE as u8 }
    }
  }
}