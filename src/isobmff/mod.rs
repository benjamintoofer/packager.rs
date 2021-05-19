use std::borrow::Borrow;

use crate::{error::CustomError, media::TrackType};

use self::{boxes::stsd::STSD, sample_entry::{avc_sample_entry::AVCSampleEntry, mp4a_sample_entry::MP4ASampleEntry}};

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

// NOTE (benjamintoofer@gmail.com): May want to use the handler rather than the TrackType
pub fn get_codec(track_type: TrackType, mp4: &[u8]) -> Result<String, CustomError> {
  if track_type == TrackType::VIDEO {
    let codec_type = "avc1";
    let avc_config = STSD::parse(&mp4)
      .and_then(|stsd| stsd.read_sample_entry(codec_type).map(|x|x.to_vec()))
      .map(|avc_data|AVCSampleEntry::parse(&avc_data))
      .map(|avc_sample|avc_sample.config)?;
    let codec = format!("{}.{:X}{:X}{:X}",
      codec_type, 
      avc_config.avc_profile_indication, 
      avc_config.profile_compatability,
      avc_config.avc_level_indication);
    return Ok(codec);
  } else if track_type == TrackType::AUDIO {
    let codec_type = "mp4a";
    let aac_data = STSD::parse(&mp4)
      .and_then(|stsd| stsd.read_sample_entry("mp4a").map(|x|x.to_vec()))
      .map(|mp4a_data|MP4ASampleEntry::parse(&mp4a_data))
      .map(|mp4a_sample|mp4a_sample.es_descriptor)?;
    
      let codec = format!("{}.{:X}.{}",
        codec_type, 
        aac_data.dec_config_descr.object_type_indication, 
        aac_data.dec_config_descr.audio_sepcific_info.audio_object_type);
    return Ok(codec);
  } else {
    Ok("".to_string())
  }
}

pub fn get_mime_type() -> String {
  
}
