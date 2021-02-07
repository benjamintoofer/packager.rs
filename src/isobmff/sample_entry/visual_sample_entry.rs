
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
    let mut end = start + 2;
    // Parse width
    let width = util::get_u16(data, start, end)
      .expect(format!("{}.parse.width: cannot get u16 from start = {}; end = {}",CLASS, start, end).as_ref());
    
    start = end;
    end = start + 2;
    // Parse height
    let height = util::get_u16(data, start, end)
      .expect(format!("{}.parse.height: cannot get u16 from start = {}; end = {}",CLASS, start, end).as_ref());

    start = end;
    end = start + 4;
    // Parse horiz resolution
    let horiz_resolution = util::get_u32(data, start, end)
      .expect(format!("{}.parse.horiz_resolution: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());

    start = end;
    end = start + 4;
    // Parse vert resolution
    let vert_resolution = util::get_u32(data, start, end)
      .expect(format!("{}.parse.vert_resolution: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());

    start = end + 4;
    end = start + 2;
    // Parse frame count
    let frame_count = util::get_u16(data, start, end)
      .expect(format!("{}.parse.frame_count: cannot get u16 from start = {}; end = {}",CLASS, start, end).as_ref());

    start = end;
    end = start + 1;
    // Parse compressor name size
    let compressor_name_size = util::get_u8(data, start, end)
      .expect(format!("{}.parse.compressor_name_size: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());
    
    let mut compressor_name = String::from("");
    for i in 0..compressor_name_size {
      let index = end + i as usize;
      if !data[index].is_ascii() {
        // Error("")
        todo!()
      }
      let character = data[index] as char;
      println!("{:?}",character);
      compressor_name.push(character);
    }

    start = start + 32;
    end = start + 2;
    // Parse depth
    let depth = util::get_u16(data, start, end)
      .expect(format!("{}.parse.depth: cannot get u16 from start = {}; end = {}",CLASS, start, end).as_ref());

    // Skip predefined value
    start = end;
    end = start + 2;
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
    }, end)
  }
}