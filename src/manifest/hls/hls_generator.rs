use std::{convert::TryInto, fs::ReadDir};
use std::str;

use crate::{manifest::manifest_generator::ManifestGenerator, media::{TrackInfo, MediaInfo, TrackType}};
use crate::isobmff::boxes::tfdt::TFDT;
use crate::manifest::hls::hls_writer::HLSWriter;
use crate::manifest::hls::{HLSMediaType, HLSBool};

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
          Err(err) => panic!("{}", err)
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

  // 1. Bucket tracks into their own group ids with its own track tracktype. 
  //  1a. NEED TO TEST DIFFERENT COMBOS (mutliple audio tracks/1 group; mutliple audio tracks/each group id; Combos for CC and Subtitles too) 
  // 2. Identify video tracks with groups of AUDIO, SUBTITLES, CC
  //  2a. Combine (codecs) and (bitrates) for audio and video
  // 3. Assign groupd ids of AUDIO, CC, SUBTITLES to the video stream inf
  // BONUS: Implement IFrame playlist generation
  pub fn generate_master(read_dir: ReadDir, metadata: &MediaInfo) -> String {
    let mut mp4_files_path: Vec<String> = vec![];
    for entry in read_dir {
      let file_entry = entry.unwrap();
      if file_entry.path().extension().unwrap() == "mp4" {
        mp4_files_path.push(file_entry.path().to_str().expect("Error").to_string())
      }
    }

    let audio_tracks = metadata.track_infos
      .iter()
      .filter(|ti|ti.track_type == TrackType::AUDIO);

    let video_tracks = metadata.track_infos
      .iter()
      .filter(|ti|ti.track_type == TrackType::VIDEO);

    let is_independent_segments = metadata.track_infos
      .iter()
      .filter(|ti|ti.track_type == TrackType::VIDEO)
      .all(|ti|ti.segments_start_with_i_frame == true);
      
    let mut hls_writer = HLSWriter::create_writer();
    let manifest = hls_writer.start_hls()
      .new_line()
      .comment("This manifest is created by Benjamin Toofer");
    
    // Iterate over audio tracks
    // for at in audio_tracks {
    //   manifest
    //   .media(
    //     HLSMediaType::AUDIO,
    //     at.group_id, 
    //     at.group_id, 
    //     Some("uri"), 
    //     Some(&at.language),
    //     None, 
    //     Some(HLSBool::YES),
    //     Some(HLSBool::YES), 
    //     None, 
    //     None, 
    //     None, 
    //     Some(at.audio_channels));
    // }

    // // Iterate over video tracks
    // for at in audio_tracks {
    //   manifest
    //   .stream_inf(
    //     "path",
    //     at.max_bandwidth, 
    //     at.average_bandwidth, 
    //     at.frame_rate, 
    //     None,
    //     None, 
    //     Some(HLSBool::YES),
    //     None, 
    //     at.codec, 
    //     None, 
    //     None, 
    //     Some(at.audio_channels));
    // }
    
      // .media(media_type, group_id, name, uri, language, assoc_language, default, auto_select, forced, instream_id, characteristics, channels)
    "".to_string()
  }

  // Need to know:
  // - which version of HLS
  // - if each segment starts with an Iframe (independent segments tag)
  // - if byterange
  // - maximum segment duration
  // - segment  info (path, duration)
  // TODO (benjamintoofer@gmail.com): Come back to this and finish
  pub fn generate_media_playlist(metadata: &TrackInfo) {

    // println!("{:?}", metadata);
    let mut hls_writer = HLSWriter::create_writer();

    let writer  = hls_writer.start_hls()
      .new_line()
      .comment("This manifest is created by Benjamin Toofer")
      .new_line()
      .target_duration(metadata.maximum_segment_duration as u8)
      .version(HLSVersion::_7)
      .map(metadata.path, Option::Some(metadata.init_segment.bytes), Option::Some(metadata.init_segment.offset))
      .new_line();

    for media_seg in &metadata.segments {
      writer.inf(media_seg.duration, Option::None);
      writer.byte_range(media_seg.bytes, media_seg.offset, metadata.path);
    }

    let manifest_str = writer.endlist().finish();
    
    println!("{}", manifest_str);
    println!("{:?}", metadata);
  }

  pub fn generate_i_frame_playlist() {

  }
}