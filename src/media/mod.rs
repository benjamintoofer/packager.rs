pub mod media_info_generator;

pub struct MediaInfo<'a> {
  track_infos: Vec<TrackInfo<'a>> 
}

pub struct TrackInfo<'a> {
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

pub struct SegmentInfo<'a> {
  duration: f32,
  url: &'a str,
  bytes: Option<u32>,
  offset: Option<u32>,
  start_with_i_frame: bool
}

// TODO (benjamintoofer@gmail.com): Add PartInfo when supporting LL-HLS