pub mod container;
pub mod manifest;
pub mod util;
pub mod error;
pub mod transcoder;
pub mod media;
pub mod app;
pub mod codec;

use std::{fs, str::FromStr};
use std::collections::hash_map::DefaultHasher;
use uuid::Uuid;

use manifest::hls::hls_writer;
use media::media_info_generator::MediaInfoGenerator;

use crate::container::isobmff::boxes::{ iso_box };
use crate::manifest::hls::hls_generator::HLSGenerator;
use crate::media::TrackInfo;
use crate::transcoder::ffmpeg::FFMPEG;
use crate::transcoder::bento::Bento;
use crate::transcoder::{VideoResolution,AudioSampleRates};
// use crate::app;




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

  // app::main();
  // let file_path = "./assets/v_frag.mp4";
  // let file_path = "./output/recording/1280x720_frag_audio.mp4";
  // let file_name = "ToS-4k_30sec.mp4";
  // let uuid = Uuid::new_v4();
  // let uuid = Uuid::from_str("e292b8c8-87b8-4016-9d25-567b3ab04f1c").unwrap();
  // let sizes: Vec<VideoResolution> = vec![VideoResolution::_720_30, VideoResolution::_480_30, VideoResolution::_360_30];
  // let rates: Vec<AudioSampleRates> = vec![AudioSampleRates::_96k, AudioSampleRates::_48k];
  // let output_dir = "./output";
  // let base_path = format!("{}/{}",output_dir, uuid.to_string());
  // let mut output_paths: Vec<String> = vec![];
  // construct_output_paths(&base_path, &sizes, &rates, &mut output_paths);

  // output_paths
  //   .iter()
  //   .for_each(|path| fs::create_dir_all(path).unwrap());
  
  let file_path = "/Users/benjamintoofer/Desktop/seg_2_complete_v.ts";
  // let file_path = "/Users/benjamintoofer/Desktop/testing/tos/segment-2.ts";
  tranmux_test(file_path)
    
}

fn tranmux_test(file_path: &str) {
  use container::remux::remux_ts_to_mp4;

  if let Ok(ts_file )= fs::read(file_path) {
    let temp = remux_ts_to_mp4(&ts_file);
    match temp {
        Ok(_) => {}
        Err(err) => {
          println!("{}", err);
        }
    }
  }
}

fn construct_output_paths(base_path: &str, vid_res: &Vec<VideoResolution> , aud_rates: &Vec<AudioSampleRates>, output_paths: &mut Vec<String>) {
  let track_directories: Vec<&str> = vec!["video", "audio"];
  for track in track_directories {
    // Construct the video paths
    if track == "video" {
      let mut video_paths: Vec<String> = vid_res
        .iter()
        .map(|res| {
          let temp = format!("{}/{}/{}_{}",base_path, track, res.value(), res.get_fps());
          return temp;
        })
        .collect();
      output_paths.append(&mut video_paths);
    }

    if track == "audio" {
      let mut audio_paths = aud_rates
        .iter()
        .map(|res|format!("{}/{}/{}",base_path, track, res.value()))
        .collect();
      output_paths.append(&mut audio_paths);
    }
  }
}
fn transcode_content(file_name: &str, base_output_path: &str, vid_res: &Vec<VideoResolution> , aud_rates: &Vec<AudioSampleRates>) {
  let input_dir = "./temp";
  let file_input = format!("{}/{}", input_dir, file_name);

  FFMPEG::transcode(&file_input, base_output_path, vid_res,aud_rates);

  // let mut mp4_files_path: Vec<String> = vec![];
  // let track_directories: Vec<&str> = vec!["video", "audio"];
  // for track in track_directories {
  //   let read_dir = fs::read_dir(format!("{}/{}",base_output_path, track)).unwrap();
  //   for entry in read_dir {
  //     let file_entry = entry.unwrap();
  //     if file_entry.path().extension().unwrap() == "mp4" {
  //       mp4_files_path.push(file_entry.path().to_str().expect("Error").to_string())
  //     }
  //   }
  // }
  // Bento::fragment(mp4_files_path);
}

fn segment_content(output_paths: &Vec<String>) {
  let mp4_files_path: Vec<String> = output_paths
    .iter()
    .map(|mp4_path|format!("{}/media.mp4",mp4_path))
    .collect();
  
  Bento::fragment(mp4_files_path);
}

fn generate_manifest(output_paths: &Vec<String>) {

  let mut track_infos: Vec<TrackInfo> = Vec::with_capacity(output_paths.len());
  for media_path in output_paths {
    let mp4_path = format!("{}/media_frag.mp4",media_path);
    let playlist_path = format!("{}/playlist.m3u8",media_path);
    let mp4_file = fs::read(&mp4_path);
    
    if let Ok(mp4) = mp4_file {
      let track_info = MediaInfoGenerator::get_track_info(mp4_path, &mp4).unwrap();
      let playlist = HLSGenerator::generate_media_playlist(&track_info);
      track_infos.push(track_info);
      // Write the playlist to disk. 
      // NOTE (benjamintoofer@gmail.com): This will need to be removed. This is just for dev purposes
      fs::write(&playlist_path, &playlist);
    }
    
  }
    let meida_info = MediaInfoGenerator::get_media_info(&track_infos).unwrap();
    HLSGenerator::generate_master( &meida_info);
}

