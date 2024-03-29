
pub trait MinorError {
  fn message(&self) -> String;
  fn code(&self) -> u8;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum MajorCode {
  ISOBMFF           = 0,
  TRANSPORT_STREAM  = 1,
  MANIFEST          = 2,
  UTIL              = 3,
  NAL               = 4,
  REMUX             = 5,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum ISOBMFFMinorCode {
  UNABLE_TO_FIND_BOX_ERROR  = 0,
  PARSE_BOX_ERROR           = 1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum TransportStreamMinorCode {
  PARSE_TS_ERROR           = 0,
  UNSUPPORTED_ADTS_PARSING = 1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum ManifestMinorCode {
  // TODO
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum UtilMinorCode {
  PARSING_UNSIGNED_ERROR        = 0,
  PARSING_BIT_READER_ERROR      = 1,
  INVALID_ISO_639_2_CODE_ERROR  = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum NalMinorCode {
  UNEXPTED_NAL_UNIT_LENGTH_ERROR          = 0,
  BYTE_STREAM_MISSING_START_PREFIX_ERROR  = 1,
  UKNOWN_NAL_UNIT_TYPE_ERROR              = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum RemuxMinorCode {
  MISSING_BUILDER_DEPENDENCY_ERROR = 0,
  UNKNOWN_STREAM_TYPE =  1,
}

impl MinorError for ISOBMFFMinorCode {
  fn message(&self) -> String {
    match self {
      ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR => { "Unable to find box".to_string() }
      ISOBMFFMinorCode::PARSE_BOX_ERROR => { "Error parsing isobmff box".to_string() }
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
          TransportStreamMinorCode::UNSUPPORTED_ADTS_PARSING => { "Unable to parse audio data transport stream".to_string() }
      }
    }

    fn code(&self) -> u8 {
      match self {
          TransportStreamMinorCode::PARSE_TS_ERROR => { TransportStreamMinorCode::PARSE_TS_ERROR as u8 }
          TransportStreamMinorCode::UNSUPPORTED_ADTS_PARSING => {TransportStreamMinorCode::UNSUPPORTED_ADTS_PARSING as u8 }
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
      UtilMinorCode::INVALID_ISO_639_2_CODE_ERROR => { "An error occurred attempting to parse an iso_639_2 value".to_string() }
    }
  }

  fn code(&self) -> u8 {
    match self {
      UtilMinorCode::PARSING_UNSIGNED_ERROR => { UtilMinorCode::PARSING_UNSIGNED_ERROR as u8 }
      UtilMinorCode::PARSING_BIT_READER_ERROR => { UtilMinorCode::PARSING_BIT_READER_ERROR as u8 }
      UtilMinorCode::INVALID_ISO_639_2_CODE_ERROR => { UtilMinorCode::INVALID_ISO_639_2_CODE_ERROR as u8 }
    }
  }
}
impl MinorError for NalMinorCode {
  fn message(&self) -> String {
    match self {
      NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH_ERROR => { "Unexpected NAL unit length".to_string() }
      NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX_ERROR => { "Byte stream is missing starting prefix of 0x00000001".to_string() }
      NalMinorCode::UKNOWN_NAL_UNIT_TYPE_ERROR => { "Uknown NAL Unit type".to_string() }
    }
  }

  fn code(&self) -> u8 {
    match self {
      NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH_ERROR => { NalMinorCode::UNEXPTED_NAL_UNIT_LENGTH_ERROR  as u8 }
      NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX_ERROR => { NalMinorCode::BYTE_STREAM_MISSING_START_PREFIX_ERROR as u8 }
      NalMinorCode::UKNOWN_NAL_UNIT_TYPE_ERROR => { NalMinorCode::UKNOWN_NAL_UNIT_TYPE_ERROR as u8 }
    }
  }
}

impl MinorError for RemuxMinorCode {
  fn message(&self) -> String {
    match self {
      RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR => { "Missing a dependency required for the builder".to_string() }
      RemuxMinorCode::UNKNOWN_STREAM_TYPE => { "Uknown elementary stream type".to_string() }
    }
  }

  fn code(&self) -> u8 {
    match self {
      RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR => { RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR as u8 }
      RemuxMinorCode::UNKNOWN_STREAM_TYPE => { RemuxMinorCode::UNKNOWN_STREAM_TYPE as u8 }
    }
  }
}