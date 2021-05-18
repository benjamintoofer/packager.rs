pub mod media_info_generator;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrackType {
  VIDEO,
  AUDIO,
  I_FRAME,
  SUBTITLES,
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
  track_infos: Vec<TrackInfo<'a>> 
}

#[derive(Debug)]
pub struct TrackInfo<'a> {
  track_type: TrackType,
  track_id: u8,
  group_id: &'a str,
  // Master manifest related
  codec: &'a str,
  mime_type: &'a str,
  frame_rate: f32,
  average_bandwidth: u32,
  bandwidth: u32,
  resoltuion: &'a str, // ENUM
  language: &'a str,
  audio_channels: u8,
  instream_id: &'a str, // ENUM
  uri: &'a str,
  // Playlist manifest related
  maximum_duration: u8,
  offset: u16, // Default to 0 if VOD
  stream_type: &'a str, // VOD | LIVE
  segments: Vec<SegmentInfo<'a>>,
  segments_start_with_i_frame: bool
}

#[derive(Debug)]
pub struct SegmentInfo<'a> {
  pts: u64,
  duration: f32,
  url: &'a str,
  bytes: Option<u32>,
  offset: Option<u32>,
  start_with_i_frame: bool
}

// TODO (benjamintoofer@gmail.com): Add PartInfo when supporting LL-HLS