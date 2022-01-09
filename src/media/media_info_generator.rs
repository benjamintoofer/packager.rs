use super::{InitSegmentInfo, MediaInfo, MediaSegmentInfo, TrackInfo, TrackType};
// TODO (benjamintoofer@gmail.com): Clean these imports
use crate::error::CustomError;
use crate::container::isobmff::HandlerType;
use crate::container::isobmff::boxes::{SampleFlag, hdlr::HDLR, iso_box::{find_box, get_box, get_media_start, get_init_segment_end}, sidx::{ SIDX, SIDXReference}, stsd::STSD, tkhd::TKHDReader, trun::TRUN, mvhd::MVHD, mdhd::MDHDReader};
use crate::container::isobmff::{get_codec, get_channel_count};
use crate::container::isobmff::sample_entry::avc_sample_entry::AVCSampleEntry;

const DEFAULT_AUDIO_GROUP: &str = "A1";

pub struct MediaInfoGenerator;

impl MediaInfoGenerator {
  pub fn get_media_info<'a>(track_infos: &'a Vec<TrackInfo<'a>>) -> Result<MediaInfo<'a>, CustomError> {
    let min_vid_duration = track_infos
      .iter()
      .filter(|track|track.track_type == TrackType::VIDEO)
      .min_by(|x,y|x.duration.partial_cmp(&y.duration).unwrap())
      .map(|track|track.duration)
      .unwrap();

    let is_independent_segments = track_infos
      .iter()
      .all(|track| track.segments_start_with_i_frame == true);
    
    Ok(MediaInfo {
      duration: min_vid_duration,
      is_independent_segments,
      track_infos,
    })
  }
  pub fn get_track_info<'a>(path: String, mp4: &[u8]) -> Result<TrackInfo<'a>, CustomError> {
    // General information
    // Boxes
    let sidx = SIDX::parse(&mp4)?;
    let hdlr = HDLR::parse(&mp4)?;
    let mvhd = MVHD::parse(&mp4)?;
    let mut tkhd_reader = TKHDReader::parse(&mp4)?;
    let mut mdhd_reader = MDHDReader::parse(&mp4)?;
    // Properties
    let mut offset = get_media_start(&mp4);
    let init_size = get_init_segment_end(&mp4);
    let asset_duration = mvhd.get_duration() as f32/ mvhd.get_timescale() as f32;
    let timescale = sidx.get_timescale();
    let references = sidx.get_references();
    let mut pts = sidx.get_earliest_presentation_time();
    let maximum_segment_duration = get_largest_segment_duration(&sidx);
    let mut max_bandwidth = 0u32;
    let mut total_bits = 0u32;
    let mut average_bandwidth = 0u32;
    let mut segments_start_with_i_frame = true;
    let temp_seg = MediaSegmentInfo{
      pts: 0,
      duration: 0f32,
      bandwidth: 0,
      bytes: 0,
      offset: 0,
      start_with_i_frame: false,
    };
    let mut segments: Vec<MediaSegmentInfo> = vec![temp_seg; sidx.get_references().len()];
    
    // Track information
    let track_id = tkhd_reader.get_track_id()?;
    let track_type = TrackType::handler_to_track_type(hdlr.get_handler_type());
    let mut track_duration: f32 = 0f32;
    let codec = get_codec(&track_type, &mp4)?;
    let mut frame_rate = 0f32;
    let mut sample_count = 0u32;
    let width = tkhd_reader.get_width()? as f32 / 65536.0;
    let height = tkhd_reader.get_height()? as f32 / 65536.0;
    let language = mdhd_reader.get_language()?;
    let audio_channels = if track_type == TrackType::AUDIO { get_channel_count(&mp4)? } else { 0u8 };

    // Init segment information
    let init_segment = InitSegmentInfo {
      bytes:init_size as u32,
      offset: 0,
    };

    for (index,sr) in references.iter().enumerate() {
       if sr.reference_type == true { // Skip reference types that are segment indexes (1)
        continue;
      }
      // Segment information
      let duration: f32 = sr.subsegment_duration as f32 / timescale as f32;
      let mut start_with_i_frame = MediaInfoGenerator::determine_start_with_i_frame_with_sap(sr);
      let trun = find_box("moof", offset, mp4)
          .map(TRUN::parse).unwrap()?;
      if !start_with_i_frame {
        // If we cannot determine that the fragment starts with an iframe we will need to look into the fragment's
        // trun to determine the first_sample_flags (if available)
        start_with_i_frame = MediaInfoGenerator::determine_start_with_i_frame_with_trun(&trun)
      }
      // Check if this a segment doesnt start with an iframe. This will update the track to know that
      // the track doesn't have segments that start with iframes 
      if !start_with_i_frame {
        segments_start_with_i_frame = false;
      }

      let seg_bandwidth = get_segment_bandwidth(&sr, timescale);
      let info = MediaSegmentInfo{
        pts,
        duration,
        bandwidth: seg_bandwidth,
        bytes: sr.referenced_size,
        offset: offset as u32,
        start_with_i_frame,
      };
      segments[index] = info;
      total_bits += sr.referenced_size * 8;
      if determine_segment_within_target_duration(&sr, timescale, maximum_segment_duration) {
       max_bandwidth = u32::max(seg_bandwidth,max_bandwidth);
      }

      sample_count += trun.sample_count;

      // Update
      offset += sr.referenced_size as usize;
      pts += sr.subsegment_duration as u64;
      track_duration += duration;
    }

    average_bandwidth = (total_bits as f32/ asset_duration) as u32;
    frame_rate = sample_count as f32 / asset_duration;
    
    let track_info = TrackInfo{
      track_id,
      track_type,
      audio_group_id: Some(DEFAULT_AUDIO_GROUP),
      cc_group_id: None,
      subtitle_group_id: None,
      codec,
      frame_rate,
      width,
      height,
      language,
      duration: track_duration,
      average_bandwidth,
      max_bandwidth,
      maximum_segment_duration,
      audio_channels,
      path,
      init_segment,
      segments,
      segments_start_with_i_frame
    };

    Ok(track_info)
  }

  fn determine_start_with_i_frame_with_sap(sidx_ref: &SIDXReference) -> bool {
    // Determine if this reference starts with SAP and has a SAP type of 1 or 2. Type 1 or 2
    // indicates that the sample is an Iframe
    sidx_ref.starts_with_sap && (sidx_ref.sap_type == 1 || sidx_ref.sap_type == 2)
  }

  fn determine_start_with_i_frame_with_trun(trun: &TRUN) -> bool {
    trun.first_sample_flags
      .map(|x|SampleFlag::parse(x))
      .as_mut()
      .map(|f| f.get_sample_depends_on() == 2) // Is an I-Frame
      .unwrap_or(false)
  }

  fn determine_captions_id() {

  }
}

fn get_captions(mp4: &[u8]) -> Result<STSD, CustomError> {
  let stsd = STSD::parse(mp4)?;
  let hdlr = HDLR::parse(mp4)?;
  // NOTE (benjamintoofer@gmail.com): Grabbing first mdat. Hopefully that's all we need
  let mdat = get_box("mdat", 0, mp4)?;
  if HandlerType::VIDE.eq(&hdlr.get_handler_type()) {
    let avc1_sample_entry = stsd.read_sample_entry("avc1")
      .map(AVCSampleEntry::parse)?;
    let nal_unit_size = avc1_sample_entry.config.length_size_minus_one + 1;
    // AVCSampleEntry::parse(avc1_sample_entry_data)
  } else if HandlerType::SOUN.eq(&hdlr.get_handler_type()) {
    // Do some audio stuff here
  }
  

  return Ok(stsd);
  }

fn get_largest_segment_duration(sidx: &SIDX) -> f32 {
  let timescale = sidx.get_timescale();
  let mut max_segment_duration = 0f32;
  for sr in sidx.get_references() {
    if sr.reference_type == true { // Skip reference types that are segment indexes (1)
      continue;
    }
    let segment_duration = sr.subsegment_duration as f32 / timescale as f32;
    max_segment_duration = f32::max(segment_duration, max_segment_duration);
  }
  max_segment_duration
}

fn get_segment_bandwidth(sr: &SIDXReference, timescale: u32) -> u32 {
  let segment_duration = sr.subsegment_duration as f32 / timescale as f32;
  ((sr.referenced_size as f32/ segment_duration) * 8f32) as u32
}

fn determine_segment_within_target_duration(sr: &SIDXReference, timescale: u32, max_duration: f32) -> bool {
  let segment_duration = sr.subsegment_duration as f32 / timescale as f32;
  let lower_bound = max_duration * 0.5;
  let upper_bound = (max_duration * 1.5) + 0.5;
  segment_duration <= upper_bound && segment_duration >= lower_bound
}

#[cfg(test)]
mod tests {
  use crate::container::isobmff::boxes::sidx;
  use super::*;

  #[test]
  fn test_get_segment_bandwidth() {
    let timescale = 30u32;
    let sidx_reference = SIDXReference {
      reference_type: false,
      referenced_size: 104621,
      subsegment_duration: 90,
      starts_with_sap: true,
      sap_type: 1,
      sap_delta_time: 0,
    };
    assert_eq!(get_segment_bandwidth(&sidx_reference, timescale), 278989);
  }

  #[test]
  fn test_segment_within_target_duration() {
    let max_duration = 6f32;
    let timescale = 30u32;
    let sidx_reference = SIDXReference {
      reference_type: false,
      referenced_size: 104621,
      subsegment_duration: 90,
      starts_with_sap: true,
      sap_type: 1,
      sap_delta_time: 0,
    };

    assert_eq!(determine_segment_within_target_duration(&sidx_reference, timescale, max_duration), true)
  }

  #[test]
  fn test_segment_not_within_target_duration() {
    let max_duration = 9f32;
    let timescale = 30u32;
    let sidx_reference = SIDXReference {
      reference_type: false,
      referenced_size: 104621,
      subsegment_duration: 90,
      starts_with_sap: true,
      sap_type: 1,
      sap_delta_time: 0,
    };

    assert_eq!(determine_segment_within_target_duration(&sidx_reference, timescale, max_duration), false)
  } 

  #[test]
  fn test_get_largest_segment_duration() {
    let sidx: SIDX = sidx::get_test_sidx();
    assert_eq!(get_largest_segment_duration(&sidx), 3f32);
  }
}