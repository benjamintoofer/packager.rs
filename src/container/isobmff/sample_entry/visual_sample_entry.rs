
static CLASS: &str = "VisualSampleEntry";

use crate::{error::CustomError, util};
use crate::codec::h264::sequence_parameter_set::SequenceParameterSet;

#[derive(Debug)]
pub struct VisualSampleEntry {
  width: u16,
  height: u16,
  horiz_resolution: u32,
  vert_resolution: u32,
  frame_count: u16,
  compressor_name: String,
  depth: u16,

  clean_aperture_box: Option<u32>,
  pixel_aspect_ratio_box: Option<u32>
}

impl VisualSampleEntry {
  pub fn parse(data: &[u8]) -> (VisualSampleEntry, usize) {
    let offset = 32usize;
    let mut start = offset;
    // Parse width
    let width = util::get_u16(data, start)
      .expect(format!("{}.parse.width: cannot get u16 from start = {}",CLASS, start).as_ref());
    
    start = start + 2;
    // Parse height
    let height = util::get_u16(data, start)
      .expect(format!("{}.parse.height: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 2;
    // Parse horiz resolution
    let horiz_resolution = util::get_u32(data, start)
      .expect(format!("{}.parse.horiz_resolution: cannot get u32 from start = {}",CLASS, start).as_ref());

    start = start + 4;
    // Parse vert resolution
    let vert_resolution = util::get_u32(data, start)
      .expect(format!("{}.parse.vert_resolution: cannot get u32 from start = {}",CLASS, start).as_ref());

    start = start + 8;
    // Parse frame count
    let frame_count = util::get_u16(data, start)
      .expect(format!("{}.parse.frame_count: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 2;
    // Parse compressor name size
    let compressor_name_size = util::get_u8(data, start)
      .expect(format!("{}.parse.compressor_name_size: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    let mut compressor_name = String::from("");
    for i in 0..compressor_name_size {
      let index = start + i as usize;
      if !data[index].is_ascii() {
        // Error("")
        todo!()
      }
      let character = data[index] as char;
      compressor_name.push(character);
    }
    // Compressorname is formatted in a fixed 32-byte field. We already offset it by 1 for the name length
    start = start + 31;
    // Parse depth
    let depth = util::get_u16(data, start)
      .expect(format!("{}.parse.depth: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 2;
    // Skip predefined value
    start = start + 2;
    // Parse CleanApertureBox
    // todo!();
    // Parse PixelAspectRatioBox
    // todo!();
    
      (VisualSampleEntry {
      width,
      height,
      horiz_resolution,
      vert_resolution,
      frame_count,
      compressor_name,
      depth,
      clean_aperture_box: None,
      pixel_aspect_ratio_box: None
    }, start)
  }
}

#[derive(Debug)]
pub struct VisualSampleEntryBuilder {
  sps_data: Vec<u8>,
}

impl VisualSampleEntryBuilder {

  pub fn create_builder() -> VisualSampleEntryBuilder {
    return VisualSampleEntryBuilder {
      sps_data: vec![],
    }
  }

  pub fn sps(mut self, sps_data: &[u8]) -> VisualSampleEntryBuilder {
    self.sps_data = sps_data.to_vec();
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, CustomError> {
    let sps = SequenceParameterSet::parse(&self.sps_data)?;
    let width = util::transform_usize_to_u8_array(sps.width());
    let height = util::transform_usize_to_u8_array(sps.height());
    Ok(vec![
      // int(16) pre_defined
      0x00, 0x00,
      // int(16) reserved
      0x00, 0x00,
      // int(32)[3] pre_defined
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,0x00, 0x00, 0x00, 0x00,
      // width
      width[1], width[0],
      // height
      height[1], height[0],
      // horizresolution = 0x00480000
      0x00, 0x48, 0x00, 0x00,
      // vertresolution = 0x00480000
      0x00, 0x48, 0x00, 0x00,
      // int(32) reserved
      0x00, 0x00, 0x00, 0x00,
      // frame_count = 1
      0x00, 0x01,
      // string[32] compressorname = "Toofer AVC Coding"
      0x11, 0x54, 0x6f, 0x6f, 0x66, 0x65, 0x72, 0x20, 0x41, 0x56, 0x43, 0x20, 0x43, 0x6f, 0x64, 0x69, 0x6e, 0x67,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      // depth
      0x00, 0x00,
      // int(16) pre_defined = -1
      0xFF, 0xFF,
    ])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_visual_sample_entry() {
    let expected_visual_sample_entry = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xE0, 0x01, 0x0E, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 0x54, 0x6f, 0x6f, 0x66, 0x65, 0x72, 0x20, 0x41, 0x56, 0x43, 0x20, 0x43, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF];
    let sps: [u8; 25] = [
      0x67, 0x42, 0xC0, 0x1E, 0xD9, 0x01, 0xE0, 0x8F, 0xEB, 0x01, 0x10, 0x00, 0x00, 0x03, 0x00, 0x10, 0x00, 0x00, 0x03, 0x03, 0xC0, 0xF1, 0x62, 0xE4, 0x80
    ];
    let visual_sample_entry = VisualSampleEntryBuilder::create_builder()
      .sps(&sps)
      .build()
      .unwrap();

    assert_eq!(visual_sample_entry, expected_visual_sample_entry);
  }
}