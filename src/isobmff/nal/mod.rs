pub mod nal_unit;

#[allow(non_camel_case_types)]
pub enum NALType {
  IDR_Picture,
  SEI,
  SPS,
  PPS,
}

impl NALType {
  pub fn value(&self) -> u8 {
    match self {
        NALType::IDR_Picture => {5}
        NALType::SEI => {6}
        NALType::SPS => {7}
        NALType::PPS => {8}
    }
  }
}