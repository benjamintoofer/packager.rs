use crate::{error::CustomError, media::TrackType};
use crate::isobmff::boxes::{stts::STTSReader, stsd::STSD, sidx::SIDX, trun::TRUN, mvhd::MVHD};
use crate::iso_box::{find_box, get_media_start};
use self::{sample_entry::{avc_sample_entry::AVCSampleEntry, mp4a_sample_entry::MP4ASampleEntry}};

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
pub fn get_codec(track_type: &TrackType, mp4: &[u8]) -> Result<String, CustomError> {
  if *track_type == TrackType::VIDEO {
    let codec_type = "avc1";
    let avc_config = STSD::parse(&mp4)
      .and_then(|stsd| stsd.read_sample_entry(codec_type).map(|x|x.to_vec()))
      .map(|avc_data|AVCSampleEntry::parse(&avc_data))
      .map(|avc_sample|avc_sample.config)?;
    let codec = format!("{}.{:02X}{:02X}{:02X}",
      codec_type, 
      avc_config.avc_profile_indication, 
      avc_config.profile_compatability,
      avc_config.avc_level_indication);
    return Ok(codec);
  } else if *track_type == TrackType::AUDIO {
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

pub fn get_channel_count(mp4: &[u8]) -> Result<u8, CustomError> {
  let aac_data = STSD::parse(&mp4)
    .and_then(|stsd| stsd.read_sample_entry("mp4a").map(|x|x.to_vec()))
    .map(|mp4a_data|MP4ASampleEntry::parse(&mp4a_data))
    .map(|mp4a_sample|mp4a_sample.es_descriptor)?;
  
  Ok(aac_data.dec_config_descr.audio_sepcific_info.channel_configuration)
}

pub fn get_frame_rate(mp4: &[u8]) -> Result<f32, CustomError> {
  let mut offset = get_media_start(&mp4);
  let mut sample_count = STTSReader::parse(&mp4)?.get_entry_count()?;
  let mvhd = MVHD::parse(&mp4)?;
  let asset_duration = mvhd.get_duration() as f32/ mvhd.get_timescale() as f32;
  let sidx_box = SIDX::parse(&mp4)?;
  let references = sidx_box.get_references();
  // If we can't get the number of samples from the stts box, we need to calculate the total number of
  // samples from each trun
  if sample_count == 0 {
    for sr in references {
      if sr.reference_type == true { // Skip reference types that are segment indexes (1)
        continue;
      }
      let trun = find_box("moof", offset, mp4)
          .map(TRUN::parse).unwrap()?;
      sample_count += trun.sample_count;
      offset += sr.referenced_size as usize;
    }
  }
  
  Ok(sample_count as f32 / asset_duration as f32)
}


/// Convert ISO-639-2/T language code to a name
pub fn map_iso_639_2_to_name(language_code: &String) -> String {
  match language_code.as_str() {
    "eng" => {"English".to_string()}
    _ => {"Unknown".to_string()}
  }

}

pub fn map_iso_639_2_to_639_1(language_code: &String) -> String {
  match language_code.as_str() {
    "eng" => {"en".to_string()}
    _ => {"un".to_string()}
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_map_iso_639_2_to_name() {
    assert_eq!(map_iso_639_2_to_name(String::from("eng")), String::from("English"));
    assert_eq!(map_iso_639_2_to_name(String::from("und")), String::from("Unknown"));
    assert_eq!(map_iso_639_2_to_name(String::from("random")), String::from("Unknown"));
  }

  #[test]
  fn test_map_iso_639_2_to_639_1() {
    assert_eq!(map_iso_639_2_to_639_1(String::from("eng")), String::from("en"));
    assert_eq!(map_iso_639_2_to_639_1(String::from("und")), String::from("un"));
  }
}