use super::{SegmentInfo, TrackInfo, TrackType};
// TODO (benjamintoofer@gmail.com): Clean these imports
use crate::error::CustomError;
use crate::isobmff::HandlerType;
use crate::isobmff::boxes::{SampleFlag, hdlr::HDLR, iso_box::{find_box, get_box, get_init_segment_end}, sidx::{ SIDX, SIDXReference}, stsd::STSD, tkhd::TKHDReader, trun::TRUN, mvhd::MVHD, mdhd::MDHDReader};
use crate::isobmff::{get_codec, get_channel_count};
use crate::isobmff::sample_entry::avc_sample_entry::AVCSampleEntry;


pub struct MediaInfoGenerator;

impl MediaInfoGenerator {
  pub fn temp(mp4: &[u8]) -> Result<SIDX, CustomError> {

    // General information
    // Boxes
    let sidx_box = SIDX::parse(&mp4)?;
    let hdlr = HDLR::parse(&mp4)?;
    let mvhd = MVHD::parse(&mp4)?;
    let mut tkhd_reader = TKHDReader::parse(&mp4)?;
    let mut mdhd_reader = MDHDReader::parse(&mp4)?;
    // Properties
    let mut offset = get_init_segment_end(&mp4);
    let asset_duration = mvhd.get_duration() as f32/ mvhd.get_timescale() as f32;
    let timescale = sidx_box.get_timescale();
    let references = sidx_box.get_references();
    let mut pts = sidx_box.get_earliest_presentation_time();
    let maximum_segment_duration = get_largest_segment_duration(&sidx_box);
    let mut max_bandwidth = 0u32;
    let mut total_bits = 0u32;
    let mut average_bandwidth = 0u32;
    let temp_seg = SegmentInfo{
      pts: 0,
      duration: 0f32,
      bandwidth: 0,
      bytes: None,
      offset: None,
      start_with_i_frame: false,
    };
    let mut segments: Vec<SegmentInfo> = vec![temp_seg; sidx_box.get_references().len()];
    println!("MAX DURATION -> {}", maximum_segment_duration);
    
    // Track information
    let track_id = tkhd_reader.get_track_id()?;
    let track_type = TrackType::handler_to_track_type(hdlr.get_handler_type());
    let group_id ="something";
    let codec = get_codec(&track_type, &mp4)?;
    let mut frame_rate = 0f32;
    let mut sample_count = 0u32;
    let width = tkhd_reader.get_width()? as f32 / 65536.0;
    let height = tkhd_reader.get_height()? as f32 / 65536.0;
    let language = mdhd_reader.get_language()?;
    let audio_channels = if track_type == TrackType::AUDIO { get_channel_count(&mp4)? } else { 0u8 };

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

      let seg_bandwidth = get_segment_bandwidth(&sr, timescale);
      let info = SegmentInfo{
        pts,
        duration,
        bandwidth: seg_bandwidth,
        bytes: Option::Some(sr.referenced_size),
        offset: Option::Some(offset as u32),
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
        
    }

    average_bandwidth = (total_bits as f32/ asset_duration) as u32;
    frame_rate = sample_count as f32 / asset_duration;
    
    let track_info = TrackInfo{
      track_id,
      track_type,
      group_id,
      codec: codec.as_ref(),
      frame_rate,
      width,
      height,
      language,
      average_bandwidth,
      max_bandwidth,
      maximum_segment_duration,
      audio_channels,
      segments,
    };

    println!("STATUS {:?}", track_info);
    Ok(sidx_box)
  }

  pub fn get_captions(mp4: &[u8]) -> Result<STSD, CustomError> {
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
  // pub fn extract_media_info_from_mp4(mp4: &[u8]) -> MediaInfo {

  // }

  // fn track_info(mp4: &[u8]) -> TrackInfo {

  // }

  // fn segment_info(mp4: &[u8]) -> SegmentInfo {

  // }

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
  (sr.referenced_size as f32/ segment_duration) as u32 * 8
}

fn determine_segment_within_target_duration(sr: &SIDXReference, timescale: u32, max_duration: f32) -> bool {
  let segment_duration = sr.subsegment_duration as f32 / timescale as f32;
  let lower_bound = max_duration * 0.5;
  let upper_bound = (max_duration * 1.5) + 0.5;
  segment_duration <= upper_bound && segment_duration >= lower_bound
}