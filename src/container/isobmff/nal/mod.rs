pub mod nal_unit;

use crate::{error::{CustomError, construct_error, error_code::NalMinorCode}, util};
use std::fmt::Debug;

#[derive(Clone)]
pub struct NalRep {
  pub nal_unit: Vec<u8>,
  pub dts: u64,
  pub pts: u64,
}

impl Debug for NalRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
        .key(&"pts: ").value(&self.pts)
        .key(&"dts: ").value(&self.dts)
        .key(&"nal_unit: ").value(&self.nal_unit.len())
        .finish()
    }
}

impl NalRep {
  /// Generate a nal unit based off of the nal bytestream
  fn from_annex_b(&self, nal_header_size: usize) -> Vec<u8>{
    let nal_size = self.nal_unit.len() + nal_header_size;
    let nal_size_array = util::transform_u32_to_u8_array(nal_size as u32).to_vec();
    [
      vec![nal_size_array[3],nal_size_array[2],nal_size_array[1],nal_size_array[0]],
      self.nal_unit
    ].concat()
  }
  /// Generate a nal bytestream
  fn to_annex_b() -> Vec<u8> {
    todo!("Implement the NalRep.to_annex_b")
  }
}

// ffmpeg -i in.264 -c copy -bsf:v trace_headers -f null - 2> NALUS.txt
#[allow(non_camel_case_types)]
pub enum NALType {
  Non_IDR_Picture, // Non independent picture
  IDR_Picture, // Independent _ refresh 
  SEI, // Supplemental enhancement information
  SPS, // Sequence parameter set 
  PPS, // Picture parameter set
  AUD, // Access unit delimiter 
}

impl NALType {
  pub fn value(&self) -> u8 {
    match self {
      NALType::Non_IDR_Picture => {1}
      NALType::IDR_Picture => {5}
      NALType::SEI => {6}
      NALType::SPS => {7}
      NALType::PPS => {8}
      NALType::AUD => {9}
    }
  }

  pub fn get_type(val: u8) -> Result<NALType, CustomError> {
    match val {
      1 => Ok(NALType::Non_IDR_Picture),
      5 => Ok(NALType::IDR_Picture),
      6 => Ok(NALType::SEI),
      7 => Ok(NALType::SPS),
      8 => Ok(NALType::PPS),
      9 => Ok(NALType::AUD),
      _ => Err(
        construct_error(
          crate::error::error_code::MajorCode::NAL,
          Box::new(NalMinorCode::UKNOWN_NAL_UNIT_TYPE_ERROR),
          format!("NAL Type not supported: {}", val),
          file!(),
          line!())
      )
    }
  }
}

//  access_unit_delimiter_rbsp