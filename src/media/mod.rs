pub mod media_info_generator;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrackType {
  VIDEO,
  AUDIO,
  I_FRAME,
  SUBTITLES,
}

pub enum ContainerFormat {
  MP4,
  MP2TS,
  WEBM
}

impl TrackType {
  pub fn handler_to_track_type(val: u32) -> TrackType {
    match val {
      0x76696465 =>{ TrackType::VIDEO }
      0x736f756e => { TrackType::AUDIO }
      _ => { TrackType::VIDEO }
    }
  }
}
#[derive(Debug)]
pub struct MediaInfo<'a> {
  duration: f32, // ms
  track_infos: Vec<TrackInfo<'a>>,
}

#[derive(Debug)]
pub struct TrackInfo<'a> {
  track_type: TrackType,
  track_id: u32,
  group_id: &'a str,
  // Master manifest related
  codec: &'a str,
  frame_rate: f32,
  average_bandwidth: u32,
  max_bandwidth: u32,
  width: u32,
  height: u32,
  language: String,
  // audio_channels: u8,
  // instream_id: &'a str,
  // Playlist manifest related
  maximum_segment_duration: f32,
  // offset: u16, // Default to 0 if VOD
  // stream_type: &'a str, // VOD | LIVE
  segments: Vec<SegmentInfo>,
  // segments_start_with_i_frame: bool
}

#[derive(Debug, Clone)]
pub struct SegmentInfo {
  pts: u64,
  duration: f32,
  bandwidth: u32,
  bytes: Option<u32>,
  offset: Option<u32>,
  start_with_i_frame: bool
}

// TODO (benjamintoofer@gmail.com): Add PartInfo when supporting LL-HLS