
// "Independent and Disposable Samples Box"
use std::str;

use crate::{iso_box::{IsoBox, IsoFullBox, find_box}, util::bit_reader::BitReader};
use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}};
use crate::container::isobmff::nal::NalRep;
use crate::util;

static CLASS: &str = "TRUN";

#[derive(Debug, Eq)]
struct Sample {
  // All optional fields
  sample_duration: Option<u32>,
  sample_size: Option<u32>,
  sample_flags: Option<u32>,
  sample_composition_time_offset: Option<i32>,
}

impl PartialEq for Sample {
  fn eq(&self, other: &Self) -> bool {
    self.sample_duration == other.sample_duration &&
    self.sample_size == other.sample_size &&
    self.sample_flags == other.sample_flags && 
    self.sample_composition_time_offset == other.sample_composition_time_offset
  }
}

#[derive(Debug, Eq)]
pub struct TRUN {
  size: u32,
  box_type: String,
  version: u8,                    
  flags: u32,                     // u24
  pub sample_count: u32,
  // Optional fields
  data_offset: Option<i32>,
  pub first_sample_flags: Option<u32>,
  samples: Vec<Sample>
}

impl IsoBox for TRUN {
  fn get_size(&self) -> u32 {
    self.size
  }

  fn get_type(&self) -> &String {
    &self.box_type
  }
}

impl IsoFullBox for TRUN {
  fn get_version(&self) -> u8 {
    self.version
  }

  fn get_flags(&self) -> u32 {
    self.flags
  }
}

impl PartialEq for TRUN {
  fn eq(&self, other: &Self) -> bool {
    self.size == other.size &&
    self.flags == other.flags &&
    self.sample_count == other.sample_count
  }
}

impl TRUN {
  pub fn parse(moof: &[u8]) -> Result<TRUN, CustomError> {
    let trun_option = find_box("traf", 8, moof)
      .and_then(|traf|find_box("trun", 8, traf));
    
    if let Some(trun_data) = trun_option {
      Ok(TRUN::parse_trun(trun_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  pub fn parse_trun(trun_data: &[u8]) -> Result<TRUN, CustomError> {
    let mut start = 0usize;

    // Parse size
    let size = util::get_u32(trun_data, start)?;

    start += 4;
    let end = start + 4;
    let box_type = str::from_utf8(trun_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!("{}", err),
    };

    // Parse flags
    start += 4;
    let flags = util::get_u32(trun_data, start)? & 0xFFFFFF;

    start += 4;
    let sample_count = util::get_u32(trun_data, start)?;
    start += 4;

    // data-offset-present
    let mut data_offset:Option<i32> = Option::None;
    if (flags & 0x000001) != 0 {
      data_offset = Option::Some(util::get_i32(trun_data, start)?);
      start += 4;
    }
    
    // first-sample-flags-present
    let mut first_sample_flags:Option<u32> = Option::None;
    if (flags & 0x000004) != 0 {
      first_sample_flags = Option::Some(util::get_u32(trun_data, start)?);
      start += 4;
    }

    let mut samples: Vec<Sample> = vec![];
    for _ in 0..sample_count {
      // sample-duration-present
      let mut sample_duration:Option<u32> = Option::None;
      if (flags & 0x000100) != 0 {
        sample_duration = Option::Some(util::get_u32(trun_data, start)?);
        start += 4;
      }

      // sample-size-present
      let mut sample_size:Option<u32> = Option::None;
      if (flags & 0x000200) != 0 {
        sample_size = Option::Some(util::get_u32(trun_data, start)?);
        start += 4;
      }

      // sample-flags-present
      let mut sample_flags:Option<u32> = Option::None;
      if (flags & 0x000400) != 0 {
        sample_flags = Option::Some(util::get_u32(trun_data, start)?);
        start += 4;
      }

      // sample-composition-time-offsets-present
      let mut sample_composition_time_offset:Option<i32> = Option::None;
      if (flags & 0x000800) != 0 {
        sample_composition_time_offset = Option::Some(util::get_i32(trun_data, start)?);
        start += 4;
      }

      samples.push(Sample {
        sample_duration,
        sample_size,
        sample_flags,
        sample_composition_time_offset
      })
    }
    
    Ok(TRUN {
      size,
      box_type,
      version: 0,
      flags,
      sample_count,
      data_offset,
      first_sample_flags,
      samples,
    })
  }

  pub fn set_data_offset(moof: &[u8], data_offset: usize) -> Result<(), CustomError> {
    let trun_option = find_box("traf", 8, moof)
      .and_then(|traf|find_box("trun", 8, traf));
    
    if let Some(trun_data) = trun_option {
      let mut start = 8usize; //Skip header
      let flags = util::get_u32(trun_data, start)? & 0xFFFFFF;

      start += 8;
      if (flags & 0x000001) != 0 {
        let end = start + 4;
        let do_array = util::transform_usize_to_u8_array(data_offset);
        trun_data.to_vec().splice(start..end, vec![do_array[3],do_array[2],do_array[1],do_array[0]]);
      }
    } 
    Ok(())
  }
}

pub struct TRUNBuilder {
  version: usize,
  flags: usize,
  samples: Vec<NalRep>,
}

impl TRUNBuilder {
  pub fn create_builder() -> TRUNBuilder {
    TRUNBuilder{
      version: 0,
      flags: 0,
      samples: vec![],
    }
  }

  pub fn samples(mut self, samples: Vec<NalRep>) -> TRUNBuilder {
    self.samples = samples;
    self
  }

  pub fn version(mut self, version: usize) -> TRUNBuilder {
    self.version = version;
    self
  }

  pub fn flags(mut self, flags: usize) -> TRUNBuilder {
    self.flags = flags;
    self
  }

  pub fn build(&self) -> Vec<u8> {
    let version_array = util::transform_usize_to_u8_array(self.version);
    let flags_array = util::transform_usize_to_u8_array(self.flags);
    let sample_count_array = util::transform_usize_to_u8_array(self.samples.len());

    let calculated_sample_size = self.calculate_sample_size(self.flags);
    let all_samples_size = calculated_sample_size * self.samples.len();
    let sample_data = TRUNBuilder::create_sample_data(&self.samples, calculated_sample_size, self.flags, self.version);

    [
      vec![
        // Size
        0x00, 0x00, 0x00, 0x10,
        // trun
        0x74, 0x72, 0x75, 0x6E,
        // version
        version_array[0],
        // flag
        flags_array[2], flags_array[1], flags_array[0],
        // sample_count
        sample_count_array[3], sample_count_array[2], sample_count_array[1], sample_count_array[0],
        // data_offset (optional but it is required for CMAF)
        0x00, 0x00, 0x00, 0x00, // Can't determine this at the time of building the box. Set it later
      ],
      sample_data
    ].concat()
  }

  fn calculate_sample_size(&self, flags: usize) -> usize {
    let mut calc_size = 0usize;
    if flags & 0x000100 != 0{ // sample-duration-present
      calc_size += 4;
    }
    if flags & 0x000200 != 0 { // sample-size-present
      calc_size += 4;
    }
    if flags & 0x000400 != 0 { // sample-flags-present
      calc_size += 4;
    }
    if flags & 0x000800 != 0 { // sample-composition-time-offsets-present
      calc_size += 4;
    }
    calc_size
  }

  fn create_sample_data(samples: &Vec<NalRep>, sample_size: usize, flags: usize, version: usize) -> Vec<u8> {
    let total_sample_size = samples.len() * sample_size;
    let mut data: Vec<u8> = vec![0; total_sample_size];
    let mut offset = 0usize;
    for nal in samples.iter() {
      let sample = TRUNBuilder::create_sample(nal, sample_size, flags, version, 0, 0);
      let end = offset + sample_size;
      data.splice(offset..end, sample);
      offset = end;
    }

    data
  }

  pub fn create_sample(sample: &NalRep, sample_size: usize, flags: usize, version: usize, duration: usize, sample_flag: usize) -> Vec<u8>{
    let mut sample_data = vec![0u8; sample_size];
    let mut offset = 0usize;
    if flags & 0x000100 != 0{ // sample-duration-present
      todo!("Implement sample-duration in create_sample");
      let duration_array = util::transform_usize_to_u8_array(duration);
      let end = offset + 4;
      sample_data
        .splice(offset..end, vec![duration_array[3], duration_array[2], duration_array[1], duration_array[0]]);
      offset = end;
    }
    if flags & 0x000200 != 0 { // sample-size-present
      let size_array = util::transform_usize_to_u8_array(sample.nal_unit.len() + 4);
      let end = offset + 4;
      sample_data
        .splice(offset..end, vec![size_array[3], size_array[2], size_array[1], size_array[0]]);
      offset = end;
    }
    if flags & 0x000400 != 0 { // sample-flags-present
      todo!("Implement sample-falgs in create_sample");
      let sf_array = util::transform_usize_to_u8_array(sample_flag);
      let end = offset + 4;
      sample_data
        .splice(offset..end, vec![sf_array[3], sf_array[2], sf_array[1], sf_array[0]]);
      offset = end;
    }
    if flags & 0x000800 != 0 { // sample-composition-time-offsets-present
      let data: Vec<u8>;
      if version == 1  {
        let diff = sample.pts as i32 - sample.pts as i32;
        data = util::transform_i32_to_u8_array(diff).to_vec();
      } else {
        let diff = (sample.pts - sample.pts) as usize;
        data = util::transform_usize_to_u8_array(diff).to_vec();
      }
      let end = offset + 4;
      sample_data
        .splice(offset..end, vec![data[3], data[2], data[1], data[0]]);
    }

    sample_data
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_trun() {
    let trun: [u8; 384] = [
      // Size
      0x00, 0x00, 0x01, 0x80,
      // trun
      0x74, 0x72, 0x75, 0x6E,
      0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x00, 0x5A, 0x00, 0x00, 0x01, 0xD8, 0x02, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x35, 0xAC, 0x00, 0x00, 0x01, 0x14, 0x00, 0x00, 0x00, 0xDB, 0x00, 0x00, 0x01, 0x7E,
      0x00, 0x00, 0x01, 0xBE, 0x00, 0x00, 0x01, 0xF6, 0x00, 0x00, 0x02, 0x5E, 0x00, 0x00, 0x02, 0x84,
      0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x02, 0x8D, 0x00, 0x00, 0x02, 0xC6, 0x00, 0x00, 0x02, 0x5E,
      0x00, 0x00, 0x02, 0xBC, 0x00, 0x00, 0x02, 0xB9, 0x00, 0x00, 0x02, 0xDE, 0x00, 0x00, 0x02, 0x94,
      0x00, 0x00, 0x02, 0xB1, 0x00, 0x00, 0x02, 0xE3, 0x00, 0x00, 0x02, 0xF4, 0x00, 0x00, 0x02, 0x5A,
      0x00, 0x00, 0x02, 0xD9, 0x00, 0x00, 0x02, 0x89, 0x00, 0x00, 0x02, 0xBD, 0x00, 0x00, 0x02, 0xBA,
      0x00, 0x00, 0x03, 0x4C, 0x00, 0x00, 0x02, 0x9B, 0x00, 0x00, 0x02, 0xFE, 0x00, 0x00, 0x03, 0x11,
      0x00, 0x00, 0x02, 0xD3, 0x00, 0x00, 0x03, 0x69, 0x00, 0x00, 0x02, 0x8E, 0x00, 0x00, 0x02, 0xE4,
      0x00, 0x00, 0x02, 0x5B, 0x00, 0x00, 0x02, 0xFB, 0x00, 0x00, 0x03, 0x31, 0x00, 0x00, 0x03, 0x23,
      0x00, 0x00, 0x05, 0x04, 0x00, 0x00, 0x04, 0x95, 0x00, 0x00, 0x05, 0x55, 0x00, 0x00, 0x05, 0x09,
      0x00, 0x00, 0x05, 0x34, 0x00, 0x00, 0x04, 0xD8, 0x00, 0x00, 0x05, 0x12, 0x00, 0x00, 0x05, 0x8B,
      0x00, 0x00, 0x04, 0xBD, 0x00, 0x00, 0x05, 0x54, 0x00, 0x00, 0x04, 0xF5, 0x00, 0x00, 0x04, 0xE1,
      0x00, 0x00, 0x05, 0x47, 0x00, 0x00, 0x05, 0xB2, 0x00, 0x00, 0x04, 0x62, 0x00, 0x00, 0x04, 0x26,
      0x00, 0x00, 0x03, 0xFC, 0x00, 0x00, 0x03, 0xBF, 0x00, 0x00, 0x03, 0x68, 0x00, 0x00, 0x03, 0x8E,
      0x00, 0x00, 0x04, 0x46, 0x00, 0x00, 0x06, 0x48, 0x00, 0x00, 0x05, 0xE9, 0x00, 0x00, 0x05, 0x2D,
      0x00, 0x00, 0x05, 0x6D, 0x00, 0x00, 0x04, 0x7C, 0x00, 0x00, 0x04, 0x93, 0x00, 0x00, 0x04, 0x9B,
      0x00, 0x00, 0x04, 0xEE, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x04, 0xDC, 0x00, 0x00, 0x04, 0xC8,
      0x00, 0x00, 0x04, 0x9F, 0x00, 0x00, 0x04, 0x87, 0x00, 0x00, 0x04, 0xA6, 0x00, 0x00, 0x04, 0x9F,
      0x00, 0x00, 0x04, 0x67, 0x00, 0x00, 0x04, 0x58, 0x00, 0x00, 0x04, 0x65, 0x00, 0x00, 0x04, 0x8F,
      0x00, 0x00, 0x04, 0x71, 0x00, 0x00, 0x05, 0x69, 0x00, 0x00, 0x05, 0x67, 0x00, 0x00, 0x05, 0x89,
      0x00, 0x00, 0x05, 0x86, 0x00, 0x00, 0x05, 0xCD, 0x00, 0x00, 0x05, 0x03, 0x00, 0x00, 0x05, 0x32,
      0x00, 0x00, 0x05, 0x58, 0x00, 0x00, 0x05, 0x30, 0x00, 0x00, 0x05, 0x07, 0x00, 0x00, 0x04, 0xDF,
      0x00, 0x00, 0x05, 0x0E, 0x00, 0x00, 0x05, 0x11
    ];
    
    let expected_trun: TRUN = TRUN{
      box_type: "trun".to_string(),
      size: 384,
      version: 0,
      flags: 0x205,
      data_offset: Option::Some(472),
      first_sample_flags: Option::Some(0x2000000),
      sample_count: 90,
      samples: vec![
        Sample {sample_size: Option::Some(13740), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(276), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(219), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(382), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(446), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(502), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(606), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(644), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(514), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(653), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(710), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(606), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(700), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(697), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(734), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(660), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(689), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(739), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(756), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(602), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(729), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(649), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(701), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(698), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(844), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(667), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(766), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(785), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(723), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(873), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(654), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(740), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(603), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(763), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(817), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(803), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1284), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1173), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1365), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1289), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1332), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1240), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1298), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1419), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1213), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1364), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1269), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1249), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1351), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1458), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1122), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1062), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1020), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(959), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(872), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(910), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1094), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1608), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1513), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1325), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1389), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1148), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1171), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1179), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1262), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1152), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1244), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1224), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1183), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1127), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1112), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1125), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1167), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1137), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1385), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1383), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1417), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1414), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1485), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1283), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1330), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1368), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1328), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1287), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1247), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1294), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
        Sample {sample_size: Option::Some(1297), sample_duration: Option::None, sample_flags: Option::None, sample_composition_time_offset: None},
      ]
    };
    assert_eq!(TRUN::parse_trun(&trun).unwrap(), expected_trun);
  }

  #[test]
  fn test_set_data_offset() {
    let moof: [u8; 400] = [
      // fake moof
      0x00, 0x00, 0x00, 0x00,
      0x6D, 0x6F, 0x6F, 0x66,
      // fake traf
      0x00, 0x00, 0x00, 0x00,
      0x74, 0x72, 0x61, 0x66,
      // Size
      0x00, 0x00, 0x01, 0x80,
      // trun
      0x74, 0x72, 0x75, 0x6E,
      0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x00, 0x5A, 0x00, 0x00, 0x01, 0xD8, 0x02, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x35, 0xAC, 0x00, 0x00, 0x01, 0x14, 0x00, 0x00, 0x00, 0xDB, 0x00, 0x00, 0x01, 0x7E,
      0x00, 0x00, 0x01, 0xBE, 0x00, 0x00, 0x01, 0xF6, 0x00, 0x00, 0x02, 0x5E, 0x00, 0x00, 0x02, 0x84,
      0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x02, 0x8D, 0x00, 0x00, 0x02, 0xC6, 0x00, 0x00, 0x02, 0x5E,
      0x00, 0x00, 0x02, 0xBC, 0x00, 0x00, 0x02, 0xB9, 0x00, 0x00, 0x02, 0xDE, 0x00, 0x00, 0x02, 0x94,
      0x00, 0x00, 0x02, 0xB1, 0x00, 0x00, 0x02, 0xE3, 0x00, 0x00, 0x02, 0xF4, 0x00, 0x00, 0x02, 0x5A,
      0x00, 0x00, 0x02, 0xD9, 0x00, 0x00, 0x02, 0x89, 0x00, 0x00, 0x02, 0xBD, 0x00, 0x00, 0x02, 0xBA,
      0x00, 0x00, 0x03, 0x4C, 0x00, 0x00, 0x02, 0x9B, 0x00, 0x00, 0x02, 0xFE, 0x00, 0x00, 0x03, 0x11,
      0x00, 0x00, 0x02, 0xD3, 0x00, 0x00, 0x03, 0x69, 0x00, 0x00, 0x02, 0x8E, 0x00, 0x00, 0x02, 0xE4,
      0x00, 0x00, 0x02, 0x5B, 0x00, 0x00, 0x02, 0xFB, 0x00, 0x00, 0x03, 0x31, 0x00, 0x00, 0x03, 0x23,
      0x00, 0x00, 0x05, 0x04, 0x00, 0x00, 0x04, 0x95, 0x00, 0x00, 0x05, 0x55, 0x00, 0x00, 0x05, 0x09,
      0x00, 0x00, 0x05, 0x34, 0x00, 0x00, 0x04, 0xD8, 0x00, 0x00, 0x05, 0x12, 0x00, 0x00, 0x05, 0x8B,
      0x00, 0x00, 0x04, 0xBD, 0x00, 0x00, 0x05, 0x54, 0x00, 0x00, 0x04, 0xF5, 0x00, 0x00, 0x04, 0xE1,
      0x00, 0x00, 0x05, 0x47, 0x00, 0x00, 0x05, 0xB2, 0x00, 0x00, 0x04, 0x62, 0x00, 0x00, 0x04, 0x26,
      0x00, 0x00, 0x03, 0xFC, 0x00, 0x00, 0x03, 0xBF, 0x00, 0x00, 0x03, 0x68, 0x00, 0x00, 0x03, 0x8E,
      0x00, 0x00, 0x04, 0x46, 0x00, 0x00, 0x06, 0x48, 0x00, 0x00, 0x05, 0xE9, 0x00, 0x00, 0x05, 0x2D,
      0x00, 0x00, 0x05, 0x6D, 0x00, 0x00, 0x04, 0x7C, 0x00, 0x00, 0x04, 0x93, 0x00, 0x00, 0x04, 0x9B,
      0x00, 0x00, 0x04, 0xEE, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x04, 0xDC, 0x00, 0x00, 0x04, 0xC8,
      0x00, 0x00, 0x04, 0x9F, 0x00, 0x00, 0x04, 0x87, 0x00, 0x00, 0x04, 0xA6, 0x00, 0x00, 0x04, 0x9F,
      0x00, 0x00, 0x04, 0x67, 0x00, 0x00, 0x04, 0x58, 0x00, 0x00, 0x04, 0x65, 0x00, 0x00, 0x04, 0x8F,
      0x00, 0x00, 0x04, 0x71, 0x00, 0x00, 0x05, 0x69, 0x00, 0x00, 0x05, 0x67, 0x00, 0x00, 0x05, 0x89,
      0x00, 0x00, 0x05, 0x86, 0x00, 0x00, 0x05, 0xCD, 0x00, 0x00, 0x05, 0x03, 0x00, 0x00, 0x05, 0x32,
      0x00, 0x00, 0x05, 0x58, 0x00, 0x00, 0x05, 0x30, 0x00, 0x00, 0x05, 0x07, 0x00, 0x00, 0x04, 0xDF,
      0x00, 0x00, 0x05, 0x0E, 0x00, 0x00, 0x05, 0x11
    ];

    TRUN::set_data_offset(&moof, 0x01020304);

    println!("{:02X?}", moof);

  }

  #[test]
  fn test_create_sample() {
    let nal_rep = NalRep{
      nal_unit: vec![0x00, 0x01, 0x02, 0x03, 0x04],
      pts: 0,
      dts: 0,
    };
    let sample_size = 16;
    let flags = 0x000F00;
    TRUNBuilder::create_sample(&nal_rep, sample_size, flags, 0, 0, 0);
  }
}