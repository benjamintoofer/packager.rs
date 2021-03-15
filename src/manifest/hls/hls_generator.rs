use std::{convert::TryInto, fs::ReadDir};
use std::str;

use crate::manifest::manifest_generator::ManifestGenerator;
use crate::isobmff::boxes::tfdt::TFDT;
use crate::manifest::hls::hls_writer::HLSWriter;

use super::HLSVersion;


pub struct HLSGenerator {

}

impl ManifestGenerator for HLSGenerator {
  
  fn generate<'a>(mp4: &[u8], timescale: u32, last_init_seg_byte: usize, asset_duration_sec: f64) -> &'a str {
    let timescale = u64::from(timescale);
    let mut prev_decode_time = std::u64::MAX;
    let mut lower_bound: usize = last_init_seg_byte;
    let manifest_str = "";

    while lower_bound < mp4.len() {
      let bound_plus_four = lower_bound + 4;
      let size = mp4[lower_bound..bound_plus_four].as_ref();
      let box_type = str::from_utf8(mp4[bound_plus_four..(bound_plus_four + 4)].as_ref());
      let size = u32::from_be_bytes(size.try_into().expect("slice with incorrect length")) as usize;
      if box_type.expect("no box type").eq("moof") {
         let tfdt = match TFDT::parse(mp4[lower_bound..(lower_bound + size)].as_ref()) {
          Ok(tfdt_box) => tfdt_box,
          Err(err) => panic!(err)
        };

        let decode_time = tfdt.get_base_media_decode_time();
        if prev_decode_time == std::u64::MAX {
          prev_decode_time = decode_time;
          lower_bound += size;
          continue;
        }

        let segment_duration: f64 = (decode_time as f64 - prev_decode_time as f64) / timescale as f64;
        println!("PREV DECODE TIME = {}", prev_decode_time);
        println!("DECODE TIME = {}", decode_time);
        println!("SEG DURATION = {}", segment_duration);
        prev_decode_time = decode_time;
      }
      lower_bound += size;
    }
    println!("ASSET DURATION = {}", asset_duration_sec);
    println!("LAST SEG  DURATION = {}", (asset_duration_sec - (prev_decode_time as f64/timescale as f64)));
    manifest_str  
  }
}

impl HLSGenerator {

  // Need to know:
  // - which veriosn of HLS
  // - All avaialable tracks (audio+langiage, video+resolution/bitrate)
  pub fn generate_master(read_dir: ReadDir) -> String {
    let mut mp4_files_path: Vec<String> = vec![];
    for entry in read_dir {
      let file_entry = entry.unwrap();
      if file_entry.path().extension().unwrap() == "mp4" {
        mp4_files_path.push(file_entry.path().to_str().expect("Error").to_string())
      }
    }

    "".to_string()
  }

  // Need to know:
  // - which veriosn of HLS
  // - if each segment starts with an Iframe (independent segments tag)
  // - if byterange
  // - maximum segment duration
  // - segment  info (path, duration)

  pub fn generate_media_playlist(metadata: &str) {

    let mut hls_writer = HLSWriter::createWriter();
    let manifest_str = hls_writer.start_hls()
      .new_line()
      .comment("This manifest is created by Benjamin Toofer")
      .new_line()
      .target_duration(6)
      .version(HLSVersion::_7)
      .map("init.mp4", Option::None, Option::None)
      .new_line()
      .inf(6.006, Option::Some("segment_0.mp4"))
      .inf(6.006, Option::Some("segment_1.mp4"))
      .inf(6.006, Option::Some("segment_2.mp4"))
      .inf(6.006, Option::Some("segment_3.mp4"))
      .inf(6.006, Option::Some("segment_4.mp4"))
      .endlist()
      .finish();
    
    println!("{}", manifest_str);
  }

  pub fn generate_i_frame_playlist() {

  }
}