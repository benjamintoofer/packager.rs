
static CLASS: &str = "VisualSampleEntry";

use crate::util;
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
      .expect(format!("{}.parse.compressor_name_size: cannot get u32 from start = {}",CLASS, start).as_ref());

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

    start = start + 32;
    // Parse depth
    let depth = util::get_u16(data, start)
      .expect(format!("{}.parse.depth: cannot get u16 from start = {}",CLASS, start).as_ref());

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