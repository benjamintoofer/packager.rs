use std::{ fs };

use media::media_info_generator::MediaInfoGenerator;

use crate::isobmff::boxes::{ iso_box };
use crate::manifest::hls::hls_generator::HLSGenerator;

use crate::transcoder::ffmpeg::FFMPEG;
use crate::transcoder::bento::Bento;
use crate::transcoder::VideoResolution;


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
  let file_path = "./assets/v_frag.mp4";
  // generate_content();

  
  // HLSGenerator::generate_media_playlist("");
  let mp4_file = fs::read(file_path);
  if let Ok(mp4) = mp4_file {
    let track_info = MediaInfoGenerator::temp(&mp4).unwrap();
    track_info
    // let sidx_box = sidx::SIDX::parse(&mp4).expect("whtever");
    // let mvhd_box = mvhd::MVHD::parse(&mp4).expect("whatever mvhd");
    // let mvhd_timescale = mvhd_box.get_timescale() as f64;
    // let mvhd_duration = mvhd_box.get_duration() as f64;
    // let offset = iso_box::get_init_segment_end(&mp4);
    // println!("SIDX timescale: {}; MVHD DUR: {}; MVHD TIMESCALE: {}, ASSET DUR: {}",sidx_box.get_timescale(), mvhd_duration, mvhd_timescale, mvhd_duration / mvhd_timescale);
    // HLSGenerator::generate(&mp4, sidx_box.get_timescale(), offset, mvhd_duration / mvhd_timescale);
    // // Need all bitrates
    // HLSGenerator::generate_master();
    // let stsd = stsd::STSD::parse(&mp4).expect("whatever stsd");
    // print!("{:#?}", stsd.get_samples_length());
  } else {
      // let mut error_message = "main: Could not open file = ".to_owned();
      // error_message.push_str(file_path);
      // eprintln!("{}", error_message);
      // process::exit(1);
  }
    
}
fn generate_content() {
  let file_input = "./temp/recording.mp4";
  let output_dir = "./output";
  let output = format!("{}/recording",output_dir);

  fs::create_dir_all(&output).unwrap();
  
  let sizes: Vec<VideoResolution> = vec![VideoResolution::_720, VideoResolution::_480, VideoResolution::_360];
  FFMPEG::transcode(file_input, &output, sizes);

  let mut mp4_files_path: Vec<String> = vec![];
  let read_dir = fs::read_dir(&output).unwrap();
  for entry in read_dir {
    let file_entry = entry.unwrap();
    if file_entry.path().extension().unwrap() == "mp4" {
      mp4_files_path.push(file_entry.path().to_str().expect("Error").to_string())
    }
  }
  Bento::fragment(mp4_files_path);
}

fn generate_manifest() {
  let output_dir = "./output";
  let output = format!("{}/recording",output_dir);
  let read_dir = fs::read_dir(&output).unwrap();
  HLSGenerator::generate_master(read_dir);
}
