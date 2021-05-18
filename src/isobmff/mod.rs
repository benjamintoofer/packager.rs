use std::borrow::Borrow;

use crate::{error::CustomError, media::TrackType};

use self::{boxes::stsd::STSD, sample_entry::avc_sample_entry::AVCSampleEntry};

pub mod boxes;
pub mod sample_entry;
pub mod configuration_records;
pub mod descriptors;
pub mod nal;

#[derive(Debug)]
pub enum HandlerType {
  VIDE,
  SOUN,
  HINT,
  META,
  AUXV
}

impl PartialEq<u32> for HandlerType {
  fn eq(&self, other: &u32) -> bool {
    match self {
        HandlerType::VIDE => 0x76696465 == *other,
        HandlerType::SOUN => 0x736F756E == *other,
        HandlerType::HINT => 0x68696e74 == *other,
        HandlerType::META => 0x6d657461 == *other,
        HandlerType::AUXV => 0x61757876 == *other
    }
  }
}

pub fn get_codec(track_type: TrackType, mp4: &[u8]) -> Result<u32, CustomError> {
  if track_type == TrackType::VIDEO {
    println!("-------------------PLEASE------------------");
    let codec_type = "avc1";
    let temp = STSD::parse(&mp4)
      .and_then(|stsd| stsd.read_sample_entry(codec_type).map(|x|x.to_vec()))
      .map(|avc_data|AVCSampleEntry::parse(&avc_data))
      .map(|avc_sample|avc_sample.config)?;
    println!("-------------------HELLO------------------");
    let codec = format!("{}.{:X}{:X}{:X}",codec_type, temp.avc_profile_indication, temp.profile_compatability, temp.avc_level_indication);
    println!("-------------------HELLO------------------");
    println!("CODEC: {}", codec);
  } else if track_type == TrackType::AUDIO {
    STSD::parse(&mp4).map(|stsd| stsd.read_sample_entry("mp4a").map(|x|x.to_vec()));
    AVCSampleEntry::parse(&mp4);
  } else {

  }
  Ok(0u32)
}
