use crate::manifest::manifest_generator::ManifestGenerator;
use crate::isobmff::boxes::tfdt::TFDT;

use std::convert::TryInto;
use std::str;


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