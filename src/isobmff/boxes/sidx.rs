use std::str;
use std::fmt::Debug;
use std::cmp::Eq;
use std::cmp::PartialEq;

use crate::{error::{CustomError, construct_error, error_code::{ISOBMFFMinorCode, MajorCode}}, iso_box::IsoFullBox};
use crate::iso_box::IsoBox;
use crate::iso_box::find_box;

use crate::util;

static CLASS: &str = "SIDX";

#[derive(Eq)]
pub struct SIDXReference {
 pub reference_type: bool,      // u1
 pub referenced_size: u32,      // u31
 pub subsegment_duration: u32,
 pub starts_with_sap: bool,     // u1
 pub sap_type: u8,              // u3
 pub sap_delta_time: u32        // u28
}

impl Debug for SIDXReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_map()
        .key(&"type: ").value(&self.reference_type)
        .key(&"size: ").value(&self.referenced_size)
        .key(&"subsegment_duration: ").value(&self.subsegment_duration)
        .key(&"starts_with_sap: ").value(&self.starts_with_sap)
        .key(&"sap_type: ").value(&self.sap_type)
        .key(&"sap_delta_time: ").value(&self.sap_delta_time)
        .finish()
    }
}

impl PartialEq for SIDXReference {
  fn eq(&self, other: &Self) -> bool {
        self.referenced_size == other.referenced_size
  }
}

#[derive(Eq)]
pub struct SIDX {
  size: u32,
  box_type: String,
  version: u8,
  reference_id: u32,
  timescale: u32,
  earliest_presentation_time: u64,
  first_offset: u64,
  reference_count: u16,
  references: Vec<SIDXReference>
}

impl IsoBox for SIDX {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for SIDX {
  fn get_version(&self) -> u8 {
    self.version
  }

  fn get_flags(&self) -> u32 {
    0u32
  }
}

impl Debug for SIDX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_map()
        .key(&"type: ").value(&self.box_type)
        .key(&"size: ").value(&self.size)
        .key(&"version: ").value(&self.version)
        .key(&"reference ID: ").value(&self.reference_id)
        .key(&"reference count: ").value(&self.reference_count)
        .key(&"timescale: ").value(&self.timescale)
        .key(&"earliest presentation time: ").value(&self.earliest_presentation_time)
        .key(&"first offset: ").value(&self.first_offset)
        .key(&"references: ").value(&self.references)
        .finish()
    }
}

impl PartialEq for SIDX {
  fn eq(&self, other: &Self) -> bool {
      self.size == other.size && 
      self.references.len() == other.references.len() &&
      self.references.iter().all(|x| other.references.iter().any(|y| y.eq(x)))
  }
}

// Implement SIDX member methods
impl SIDX {
  pub fn get_first_offset(&self) -> u64 {
    self.first_offset
  }

  pub fn get_earliest_presentation_time(&self) -> u64 {
    self.earliest_presentation_time
  }

  pub fn get_timescale(&self) -> u32 {
    self.timescale
  }

  pub fn get_references(&self) -> &Vec<SIDXReference> {
    &self.references
  }
}

// Implement SIDX static methods
impl SIDX {
  pub fn parse(mp4: &[u8]) -> Result<SIDX, CustomError> {
    let sidx_option = find_box("sidx", 0, mp4);
    
    if let Some(sidx_data) = sidx_option {
      Ok(SIDX::parse_sidx(sidx_data)?)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", CLASS),
        file!(),
        line!()))
    }
  }

  fn parse_sidx(sidx_data: &[u8]) -> Result<SIDX, CustomError> {
    let mut start = 0usize;
    // Parse size
    let size = util::get_u32(sidx_data, start)?;

    start = start + 4;
    let end = start + 4;
    let box_type = str::from_utf8(sidx_data[start..end].as_ref()); 
    
    let box_type= match box_type {
      Ok(box_type_str) => String::from(box_type_str),
      Err(err) => panic!(err),
    };

    // Parse version
    start = start + 4;
    let version = util::get_u8(sidx_data, start)?;

    // Parse refernce ID
    start = start + 4;
    let reference_id = util::get_u32(sidx_data, start)?;

    // Parse timescale
    start = start + 4;
    let timescale = util::get_u32(sidx_data, start)?;

    // Parse earliest presentation time
    start = start + 4;
    let earliest_presentation_time: u64;
    if version == 0 {
      earliest_presentation_time = u64::from(util::get_u32(sidx_data, start)?);
      start = start + 4;
    } else {
      earliest_presentation_time = util::get_u64(sidx_data, start)?;
      start = start + 8;
    }
    
    // Parse first offset
    let first_offset: u64;
    if version == 0 {
      first_offset = u64::from(util::get_u32(sidx_data, start)?);
        start = start + 4;
    } else {
      first_offset = util::get_u64(sidx_data, start)?;
      start = start + 8;
    }
    

    // Parse reference count
    start = start + 2;
    let reference_count = util::get_u16(sidx_data, start)?;
    start = start + 2;
    let mut references:Vec<SIDXReference> = vec![];
    for _ in 0..reference_count {
      let four_bytes = util::get_u32(sidx_data, start)?;
      let reference_type = (four_bytes & 0x80000000) != 0;
      let referenced_size = four_bytes & !0x80000000;
      
      // Parse sub segment duration
      start = start + 4;
      let subsegment_duration = util::get_u32(sidx_data, start)?;

      // Parse starts_with_sap, sap_type, sap_delta_time
      start = start + 4;
      let four_bytes = util::get_u32(sidx_data, start)?;
      
      start = start + 4;
      let starts_with_sap = (four_bytes & 0x80000000) != 0;
      let sap_type = ((four_bytes & 0x70000000) >> 28) as u8;
      let sap_delta_time = four_bytes & 0xFFFFFFF;

      if sap_delta_time != 0 {
        println!("WARNING: sap_delta_time is {}. This is currently not handled",sap_delta_time);
      }

      let sidx_reference: SIDXReference = SIDXReference{
        reference_type: reference_type,
        referenced_size: referenced_size,
        subsegment_duration: subsegment_duration,
        starts_with_sap: starts_with_sap,
        sap_type: sap_type,
        sap_delta_time: sap_delta_time
      };
      references.push(sidx_reference);
    }
    
    Ok(SIDX{
      size: size,
      box_type: box_type,
      version: version,
      reference_id: reference_id,
      timescale: timescale,
      earliest_presentation_time: earliest_presentation_time,
      first_offset: first_offset,
      reference_count: reference_count,
      references: references,
    })
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::fs;

  #[test]
  fn test_parse_sidx() {
    let file_path = "./assets/v_frag.mp4";
  
    let expected_sidx: SIDX = SIDX{
      box_type: "sidx".to_string(),
      size: 164,
      version: 0,
      reference_id: 1,
      timescale: 30,
      earliest_presentation_time: 0,
      first_offset: 0,
      reference_count: 11,
      references: vec![
        SIDXReference{
          reference_type: false,
          referenced_size: 104621,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
        SIDXReference{
          reference_type: false,
          referenced_size: 120973,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 119315,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 125533,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 118733,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 109095,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 105545,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 110456,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 111973,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 105008,
          subsegment_duration: 90,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        },
         SIDXReference{
          reference_type: false,
          referenced_size: 4344,
          subsegment_duration: 1,
          starts_with_sap: true,
          sap_type: 0,
          sap_delta_time: 0,
        }
      ]
    };
    let mp4_file = fs::read(file_path);
    if let Ok(mp4) = mp4_file {
      assert_eq!(SIDX::parse(&mp4).unwrap(), expected_sidx);
    } else {
      panic!("mp4 file {:} cannot be opened", file_path);
    }
  }
}