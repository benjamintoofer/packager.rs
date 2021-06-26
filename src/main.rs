use std::{fs, str::FromStr};
use std::collections::hash_map::DefaultHasher;
use uuid::Uuid;

use manifest::hls::hls_writer;
use media::media_info_generator::MediaInfoGenerator;

use crate::isobmff::boxes::{ iso_box };
use crate::manifest::hls::hls_generator::HLSGenerator;

use crate::transcoder::ffmpeg::FFMPEG;
use crate::transcoder::bento::Bento;
use crate::transcoder::{VideoResolution,AudioSampleRates};


pub mod isobmff;
pub mod transport_stream;
pub mod manifest;
pub mod util;
pub mod error;
pub mod transcoder;
pub mod media;

//  1. Given a path to the asset and it's transcoded mp4's (must be fragmented for now and seperated tracks (Need to add ability to parse single mp4 with both tracks))
//  2. Iterate through each rendition, collect the correct metadata to generate master manifest
//    2.1. Common data structure for DASH and HLS (MediaInfo)
//      2.1.1. HLS per rendition - bitrate(segment), average bitrate, codec, framerate
//      2.1.2 Dash per rendition - bitrate, framerate/audioSamplingRate, codec, width + height, 
//    2.2 DASH specific data
//      2.2.1 AdaptationSet - mimeType, segmentAlign, subsegmentAlign, startWithSAP (look at what this means)


/**
 * AVCProfileIndication
66
profile_compatibility
192
AVCLevelIndication
30

PARSE AVC codec string
1. get the stsd box
2. Look for the entries (get the video entry)
3. parse avc1 box 
4. parse avcC box (AVCConfigurationBox - 14996-15 5.3.4.1.2)
5. parse AVCDecoderConfigurationRecord (14996-15 5.2.4.1.1)
5. parse AVCProfileIndication, profile_compatibility, and AVCLevelIndication to find video codec

PARSE AAC(MP4A) codec string
1. get the stsd box
2. Look for enntries (get the audio entry)
3. parse mp4a box (14996-14 5.6.1)
4. parse ESDBox (14996-1 7.2.6.5)
 */

fn main() {
  // let file_path = "./assets/v_frag.mp4";
  // let file_path = "./output/recording/1280x720_frag_audio.mp4";
  let file_name = "ToS-4k_30sec.mp4";
  let uuid = Uuid::new_v4();
  let sizes: Vec<VideoResolution> = vec![VideoResolution::_720_30, VideoResolution::_480_30, VideoResolution::_360_30];
  let rates: Vec<AudioSampleRates> = vec![AudioSampleRates::_96k, AudioSampleRates::_48k];
  // let uuid = Uuid::from_str("25fe6395-a1bb-4821-8462-eca8c45d19b0").unwrap();
  transcode_segment_content(file_name, &uuid, &sizes, &rates);

  generate_manifest(&uuid, &sizes, &rates);

  // HLSGenerator::generate_media_playlist("");
  // let mp4_file = fs::read(file_path);
  // if let Ok(mp4) = mp4_file {
  //   let track_info = MediaInfoGenerator::get_track_info(&mp4).unwrap();

    // let sidx_box = sidx::SIDX::parse(&mp4).expect("whtever");
    // let mvhd_box = mvhd::MVHD::parse(&mp4).expect("whatever mvhd");
    // let mvhd_timescale = mvhd_box.get_timescale() as f64;
    // let mvhd_duration = mvhd_box.get_duration() as f64;
    // let offset = iso_box::get_media_start(&mp4);
    // println!("SIDX timescale: {}; MVHD DUR: {}; MVHD TIMESCALE: {}, ASSET DUR: {}",sidx_box.get_timescale(), mvhd_duration, mvhd_timescale, mvhd_duration / mvhd_timescale);
    // HLSGenerator::generate(&mp4, sidx_box.get_timescale(), offset, mvhd_duration / mvhd_timescale);
    // // Need all bitrates
    // HLSGenerator::generate_master();
    // let stsd = stsd::STSD::parse(&mp4).expect("whatever stsd");
    // print!("{:#?}", stsd.get_samples_length());
  // } else {
      // let mut error_message = "main: Could not open file = ".to_owned();
      // error_message.push_str(file_path);
      // eprintln!("{}", error_message);
      // process::exit(1);
  // }
    
}
fn transcode_segment_content(file_name: &str, uuid: &Uuid, vid_res: &Vec<VideoResolution> , aud_rates: &Vec<AudioSampleRates>) {
  let input_dir = "./temp";
  let file_input = format!("{}/{}", input_dir, file_name);
  let output_dir = "./output";
  let transcode_output = format!("{}/{}",output_dir,uuid.to_string());

  fs::create_dir_all(&transcode_output).unwrap();
  FFMPEG::transcode(&file_input, &transcode_output, vid_res,aud_rates);

  let mut mp4_files_path: Vec<String> = vec![];
  let track_directories: Vec<&str> = vec!["video", "audio"];
  for track in track_directories {
    let read_dir = fs::read_dir(format!("{}/{}",&transcode_output, track)).unwrap();
    for entry in read_dir {
      let file_entry = entry.unwrap();
      if file_entry.path().extension().unwrap() == "mp4" {
        mp4_files_path.push(file_entry.path().to_str().expect("Error").to_string())
      }
    }
  }
  Bento::fragment(mp4_files_path);
}

fn generate_manifest(uuid: &Uuid, vid_res: &Vec<VideoResolution> , aud_rates: &Vec<AudioSampleRates>) {
  let output_dir = "./output";
  let media_dir = format!("{}/{}",output_dir,uuid.to_string());
  println!("UUID - {}", uuid.to_string());
  let mut mp4_files_path: Vec<String> = vec![];
  let track_directories: Vec<&str> = vec!["video", "audio"];
  for track in track_directories {
    // Get the video tracks
    if track == "video" {
      let temp = vid_res
        .iter()
        .filter_map(|res|{
          let paths = fs::read_dir(format!("{}/{}/{}_{}",&media_dir, track,res.value(), res.get_fps())).unwrap();
          paths
            .filter_map(|path| => )
        });
    }
    let read_dir = fs::read_dir(format!("{}/{}",&media_dir, track)).unwrap();
    for entry in read_dir {
      let file_entry = entry.unwrap();
      if file_entry.path().extension().unwrap() == "mp4" {
        mp4_files_path.push(file_entry.path().to_str().expect("Error").to_string())
      }
    }
    // Get the audio tracks
  }
  for mp4_path in mp4_files_path {
    let mp4_file = fs::read(&mp4_path);
    if let Ok(mp4) = mp4_file {
      let track_info = MediaInfoGenerator::get_track_info(&mp4_path, &mp4).unwrap();
      let playlist = HLSGenerator::generate_media_playlist(&track_info);
      // track_info.
      // save_manifest(, playlist)
    }
  }
}

fn save_manifest(path: &str, playlist: &str) {

}
