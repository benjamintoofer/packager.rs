enum DefaultValue {
  YES,
  NO
}

pub enum MediaTagInfo {
  AUDIO{uri: &str, assoc_language: &str, group_id: &str, name: &str, default: Option<DefaultValue>, auto_select: Option<DefaultValue>, channels: Option<&str>},
  VIDEO{uri: &str, group_id: &str, name: &str, default: Option<DefaultValue>, auto_select: Option<DefaultValue>, channels: Option<&str>},
  SUBTITLES{uri: &str, assoc_language: &str, group_id: &str, name: &str, default: Option<DefaultValue>, auto_select: Option<DefaultValue>, channels: Option<&str>},
  CLOSED_CAPTIONS{instream_id: &str, assoc_language: &str, group_id: &str, name: &str, default: Option<DefaultValue>, auto_select: Option<DefaultValue>, channels: Option<&str>}
}

pub enum StreamInfTagData {

}
pub trait MasterWriter {
  fn write_independant_segments() -> MasterWriter;
  fn write_version() -> MasterWriter;
  fn write_stream_inf(stream_inf_tag_data: StreamInfTagData) -> MasterWriter;
  fn write_i_frames_only() -> MasterWriter;
  fn write_media(media_tag_info: MediaTagInfo) -> MasterWriter;
  fn write_session_data() -> MasterWriter;
  fn write_session_key() -> MasterWriter;
}

pub trait PlaylistWriter {
  fn write_target_duration() -> PlaylistWriter;
  fn write_media_sequence() -> PlaylistWriter;
  fn write_discontinuity_sequence() -> PlaylistWriter;
  fn write_endlist() -> PlaylistWriter;
  fn write_playlist_type() -> PlaylistWriter;
  fn write_inf() -> PlaylistWriter;
  fn write_byterange() -> PlaylistWriter;
  fn write_discontinuity() -> PlaylistWriter;
  fn write_key() -> PlaylistWriter;
  fn write_map() -> PlaylistWriter;
  fn write_program_date_time() -> PlaylistWriter;
  fn write_gap() -> PlaylistWriter;
  fn write_part() -> PlaylistWriter;
  fn write_daterange() -> PlaylistWriter;
  fn write_preload_hint() -> PlaylistWriter;
}