use crate::error::{CustomError, construct_error, error_code::NalMinorCode};

pub mod nal_unit;

// ffmpeg -i in.264 -c copy -bsf:v trace_headers -f null - 2> NALUS.txt
#[allow(non_camel_case_types)]
pub enum NALType {
  IDR_Picture, // Independent _ refresh 
  SEI, // Supplemental enhancement information
  SPS, // Sequence parameter set 
  PPS, // Picture parameter set
  AUD, // Access unit delimiter 
}

impl NALType {
  pub fn value(&self) -> u8 {
    match self {
        NALType::IDR_Picture => {5}
        NALType::SEI => {6}
        NALType::SPS => {7}
        NALType::PPS => {8}
        NALType::AUD => {9}
    }
  }

  pub fn get_type(val: u8) -> Result<NALType, CustomError> {
    match val {
      5 => Ok(NALType::IDR_Picture),
      6 => Ok(NALType::SEI),
      7 => Ok(NALType::SPS),
      8 => Ok(NALType::PPS),
      9 => Ok(NALType::AUD),
      _ => Err(
        construct_error(
          crate::error::error_code::MajorCode::NAL,
          Box::new(NalMinorCode::UKNOWN_NAL_UNIT_TYPE),
          format!("NAL Type not supported: {}", val),
          file!(),
          line!())
      )
    }
  }
}

//  access_unit_delimiter_rbsp