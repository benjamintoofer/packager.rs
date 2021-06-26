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
  pub duration: f32, // ms
  pub track_infos: Vec<TrackInfo<'a>>,
}

#[derive(Debug)]
pub struct TrackInfo<'a> {
  pub track_type: TrackType,
  pub track_id: u32,
  // Master manifest related
  pub audio_group_id: Option<&'a str>,
  pub cc_group_id: Option<&'a str>,
  pub subtitle_group_id: Option<&'a str>,

  pub codec: String,
  pub frame_rate: f32,
  pub average_bandwidth: u32,
  pub max_bandwidth: u32,
  pub width: f32,
  pub height: f32,
  pub language: String,
  pub audio_channels: u8,
  // instream_id: &'a str,
  // Playlist manifest related
  pub maximum_segment_duration: f32,
  // offset: u16, // Default to 0 if VOD
  // stream_type: &'a str, // VOD | LIVE
  pub path: &'a str,
  pub init_segment: InitSegmentInfo,
  pub segments: Vec<MediaSegmentInfo>,
  pub segments_start_with_i_frame: bool
}

#[derive(Debug, Clone)]
pub struct MediaSegmentInfo {
  pts: u64,
  pub duration: f32,
  bandwidth: u32,
  pub bytes: u32,
  pub offset: u32,
  start_with_i_frame: bool,
}

#[derive(Debug, Clone)]
pub struct InitSegmentInfo {
  pub bytes: u32,
  pub offset: u32,
}
// TODO (benjamintoofer@gmail.com): Add PartInfo when supporting LL-HLS