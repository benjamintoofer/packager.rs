use std::{ fs, process };

use isobmff::sample_entry::mp4a_sample_entry;

use crate::isobmff::boxes::{ iso_box, mvhd, sidx, stsd };
use crate::manifest::hls::hls_generator;
use crate::manifest::manifest_generator::ManifestGenerator;
use crate::transport_stream::parse_transport_stream;


pub mod isobmff;
pub mod transport_stream;
pub mod manifest;
pub mod util;
pub mod error;

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
  println!("TP");
  // let file_path = "./assets/a_frag.mp4";
  
  // let mp4_file = fs::read(file_path);
  // if let Ok(mp4) = mp4_file {

  //   let sidx_box = sidx::SIDX::parse(&mp4).expect("whtever");
  //   let mvhd_box = mvhd::MVHD::parse(&mp4).expect("whatever mvhd");
  //   let mvhd_timescale = mvhd_box.get_timescale() as f64;
  //   let mvhd_duration = mvhd_box.get_duration() as f64;
  //   let offset = iso_box::get_init_segment_end(&mp4);
  //   hls_generator::HLSGenerator::generate(&mp4, sidx_box.get_timescale(), offset, mvhd_duration / mvhd_timescale);
  //   let stsd = stsd::STSD::parse(&mp4).expect("whatever stsd");
  //   print!("{:#?}", stsd.get_samples_length());
  //   // let some = stsd.read_sample_entry("avc1");
  //   let some = stsd.read_sample_entry("mp4a");
  //   match some {
  //     Some(byte_data) => {
  //       // let avc_sample_entry = avc_sample_entry::AVCSampleEntry::parse(byte_data);
  //       let mp4a_sample_entry = mp4a_sample_entry::MP4ASampleEntry::parse(byte_data);
  //       print!("{:#?}", mp4a_sample_entry);
  //     }
  //     None => {print!("NOT FOUND :(")}
  //   }
  // } else {
  //     let mut error_message = "main: Could not open file = ".to_owned();
  //     error_message.push_str(file_path);
  //     eprintln!("{}", error_message);
  //     process::exit(1);
  // }

  let file_path = "./assets/hls/media-1/segment-0.ts";
  let ts_file = fs::read(file_path);
  if let Ok(ts) = ts_file {
    parse_transport_stream(ts.as_ref());
  } else {
    let mut error_message = "main: Could not open file = ".to_owned();
    error_message.push_str(file_path);
    eprintln!("{}", error_message);
    process::exit(1);
  }
    
}

