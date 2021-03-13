use crate::transcoder::VideoResolution;
use crate::manifest::hls::{
  HLSVersion,
  HLSPlaylistType,
  HLSMediaType,
  VIDEO_RANGE,
  HDCP_LEVEL,
  HLSBool,
  CCInstreamId
};

static EXT_TAG_PREFIX: &'static str = "#EXT";

pub struct HLSWriter {
  hls_manifest_str: String
}


impl HLSWriter {
  pub fn start_hls(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}M3U\n", EXT_TAG_PREFIX).as_str());
    self
  }

  /**
   * Basic Manifest Tags.
   */

  fn version(&mut self, version: HLSVersion) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-VERSION:{}\n", EXT_TAG_PREFIX, version.value()).as_str());
    self
  }

  /**
   * Master and Playlist Manifest Tags.
   */

  fn independent(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-INDEPENDENT-SEGMENTS\n", EXT_TAG_PREFIX).as_str());  
    self
  }

  /**
   * Master Manifest Tags.
   */

  fn stream_inf(
    &mut self,
    path: &str,
    bandwidth: u32,
    average_bandwidth: Option<u32>,
    frame_rate: Option<f32>,
    hdcp_level: Option<HDCP_LEVEL>,
    allowed_cpc: Option<&str>,
    resolution: Option<VideoResolution>,
    video_range: Option<VIDEO_RANGE>,
    codecs: Option<&str>,
    // group-ids
    audio: Option<&str>,
    video: Option<&str>,
    subtitles: Option<&str>,
    closed_captions: Option<&str>,
  ) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-STREAM-INF:BANDWIDTH={}", EXT_TAG_PREFIX, bandwidth).as_str());  
    if let Some(average_bandwidth) = average_bandwidth {
      self.hls_manifest_str.push_str(format!(",AVERAGE-BANDWIDTH={}", average_bandwidth).as_str());  
    }
    if let Some(frame_rate) = frame_rate {
      self.hls_manifest_str.push_str(format!(",FRAME-RATE={}", frame_rate).as_str());  
    }
    if let Some(hdcp_level) = hdcp_level {
      self.hls_manifest_str.push_str(format!(",HDCP-LEVEL={}", hdcp_level.value()).as_str());  
    }
    if let Some(allowed_cpc) = allowed_cpc {
      self.hls_manifest_str.push_str(format!(",ALLOWED-CPC=\"{}\"", allowed_cpc).as_str());  
    }
    if let Some(resoltuion) = resolution {
      self.hls_manifest_str.push_str(format!(",RESOLUTION={}", resoltuion.value()).as_str());  
    }
    if let Some(video_range) = video_range {
      self.hls_manifest_str.push_str(format!(",VIDEO-RANGE={}", video_range.value()).as_str());  
    }
    if let Some(codecs) = codecs {
      self.hls_manifest_str.push_str(format!(",CODECS=\"{}\"", codecs).as_str());  
    }
    // Groupd ids
    if let Some(audio) = audio {
      self.hls_manifest_str.push_str(format!(",AUDIO=\"{}\"", audio).as_str());  
    }
    if let Some(video) = video {
      self.hls_manifest_str.push_str(format!(",VIDEO=\"{}\"", video).as_str());  
    }
    if let Some(subtitles) = subtitles {
      self.hls_manifest_str.push_str(format!(",SUBTITLES=\"{}\"", subtitles).as_str());  
    }
    if let Some(closed_captions) = closed_captions {
      self.hls_manifest_str.push_str(format!(",CLOSED-CAPTIONS=\"{}\"", closed_captions).as_str());  
    }
    // Add the path on the next line
    self.hls_manifest_str.push('\n');
    self.hls_manifest_str.push_str(format!("{}\n", path).as_str());  
    self
  }

  fn media(
    &mut self,
    media_type: HLSMediaType,
    group_id: &str,
    name: &str,
    uri: Option<&str>,
    language: Option<&str>,
    assoc_language: Option<&str>,
    default: Option<HLSBool>,
    auto_select: Option<HLSBool>,
    forced: Option<HLSBool>,
    instream_id: Option<CCInstreamId>,          // ONLY CLOSED-CAPTIONS
    characteristics: Option<&str>,
    channels: Option<&str>,
  ) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-MEDIA:TYPE={},GROUP-ID=\"{}\",NAME=\"{}\"", EXT_TAG_PREFIX, media_type.value(), group_id, name).as_str());  
    if let Some(language) = language {
      self.hls_manifest_str.push_str(format!(",LANGUAGE=\"{}\"", language).as_str());  
    }
    if let Some(assoc_language) = assoc_language {
      self.hls_manifest_str.push_str(format!(",ASSOC-LANGUAGE=\"{}\"", assoc_language).as_str());  
    }
    if let Some(default) = default {
      self.hls_manifest_str.push_str(format!(",DEFAULT={}", default.value()).as_str());  
    }
    if let Some(auto_select) = auto_select {
      self.hls_manifest_str.push_str(format!(",AUTO-SELECT={}", auto_select.value()).as_str());  
    }
    if let Some(forced) = forced {
      self.hls_manifest_str.push_str(format!(",FORCED={}", forced.value()).as_str());  
    }
    if let Some(instream_id) = instream_id {
      self.hls_manifest_str.push_str(format!(",INSTREAM-ID=\"{}\"", instream_id.value()).as_str());  
    }
    if let Some(characteristics) = characteristics {
      self.hls_manifest_str.push_str(format!(",CHARACTERISTICS=\"{}\"", characteristics).as_str());  
    }
    if let Some(channels) = channels {
      self.hls_manifest_str.push_str(format!(",CHANNELS=\"{}\"", channels).as_str());  
    }
    if let Some(uri) = uri {
      self.hls_manifest_str.push_str(format!(",URI=\"{}\"", uri).as_str());  
    }
    self.hls_manifest_str.push('\n');
    self
  }

  fn i_frame_stream_inf(
    &mut self,
    uri: &str,
    bandwidth: u32,
    average_bandwidth: Option<u32>,
    hdcp_level: Option<HDCP_LEVEL>,
    allowed_cpc: Option<&str>,
    resolution: Option<VideoResolution>,
    video_range: Option<VIDEO_RANGE>,
    codecs: Option<&str>,
    // group-ids
    video: Option<&str>,
  ) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-I-FRAME-STREAM-INF:BANDWIDTH={}", EXT_TAG_PREFIX, bandwidth).as_str());  
    if let Some(average_bandwidth) = average_bandwidth {
      self.hls_manifest_str.push_str(format!(",AVERAGE-BANDWIDTH={}", average_bandwidth).as_str());  
    }
    if let Some(hdcp_level) = hdcp_level {
      self.hls_manifest_str.push_str(format!(",HDCP-LEVEL={}", hdcp_level.value()).as_str());  
    }
    if let Some(allowed_cpc) = allowed_cpc {
      self.hls_manifest_str.push_str(format!(",ALLOWED-CPC=\"{}\"", allowed_cpc).as_str());  
    }
    if let Some(resoltuion) = resolution {
      self.hls_manifest_str.push_str(format!(",RESOLUTION={}", resoltuion.value()).as_str());  
    }
    if let Some(video_range) = video_range {
      self.hls_manifest_str.push_str(format!(",VIDEO-RANGE={}", video_range.value()).as_str());  
    }
    if let Some(codecs) = codecs {
      self.hls_manifest_str.push_str(format!(",CODECS=\"{}\"", codecs).as_str());  
    }
    // Group ids
    if let Some(video) = video {
      self.hls_manifest_str.push_str(format!(",VIDEO=\"{}\"", video).as_str());  
    }
    self.hls_manifest_str.push_str(format!(",URI=\"{}\"\n", uri).as_str());  
    self
  }

  /**
   * Playlist Manifest Tags
   */
  
  fn target_duration(&mut self, duration: u8) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-TARGETDURATION:{}\n", EXT_TAG_PREFIX, duration).as_str());
    self
  }

  fn media_sequence(&mut self, sequence: u16) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-MEDIA-SEQUENCE:{}\n", EXT_TAG_PREFIX, sequence).as_str());
    self
  }

  fn playlist_type(&mut self, playlist: HLSPlaylistType) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-PLAYLIST-TYPE:{}\n", EXT_TAG_PREFIX, playlist.value()).as_str());
    self
  }

  fn discontinuity_sequence(&mut self, sequence: u16) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-DISCONTINUITY-SEQUENCE:{}\n", EXT_TAG_PREFIX, sequence).as_str());
    self
  }

  fn i_frames_only(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-I-FRAMES-ONLY\n", EXT_TAG_PREFIX).as_str());
    self
  }

  fn part_inf(&mut self, part_target: f32) -> &HLSWriter {
     self.hls_manifest_str.push_str(format!("{}-X-PART-INF:PART-TARGET={}\n", EXT_TAG_PREFIX, part_target).as_str());
    self
  }

  fn endlist(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-ENDLIST\n", EXT_TAG_PREFIX).as_str());  
    self
  }

  /**
   * Media Segment Tags
   */

  fn inf(&mut self, duration: f32, uri: Option<&str>) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}INF:{}\n", EXT_TAG_PREFIX, duration).as_str());  
    if let Some(uri) = uri {
      self.hls_manifest_str.push_str(format!("{}\n", uri).as_str());  
    }
    self
  }

  fn byte_range(&mut self, bytes: u32, offset: u32, uri: &str) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-BYTERANGE:{}@{}\n{}", EXT_TAG_PREFIX, bytes, offset, uri).as_str());
    self
  }

  fn discontinuity(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-DISCONTINUITY", EXT_TAG_PREFIX).as_str());
    self
  }

  fn map(&mut self, uri: &str, bytes: u32, offset: u32) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-MAP:URI={},BYTERANGE={}@{}", EXT_TAG_PREFIX, uri, bytes, offset).as_str());
    self
  }

  fn gap(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-GAP", EXT_TAG_PREFIX).as_str());
    self
  }

  fn program_date_time(&mut self, time: &str) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-PROGRAM-DATE-TIME: {}", EXT_TAG_PREFIX, time).as_str());
    self
  }

  fn part(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-GAP", EXT_TAG_PREFIX).as_str());
    self
  }

  /**
   * Operations
   */

  fn new_line(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push('\n'); 
    self
  }

  fn finish(&self) -> &str {
    self.hls_manifest_str.as_str()
  } 
}

impl HLSWriter {
  pub fn createWriter() -> HLSWriter {
    HLSWriter{
      hls_manifest_str: String::from("")
    }
  }
}


#[cfg(test)]
mod tests {
  use super::HLSWriter;
  use super::{HLSVersion, HDCP_LEVEL, VIDEO_RANGE, VideoResolution, HLSMediaType, HLSBool, CCInstreamId};

  /**
   * Master Manifest Tags.
   */

  // VERSION
  #[test]
  fn test_hls_version() {
    let expected_manifest = "#EXT-X-VERSION:7\n";

    let mut writer = HLSWriter::createWriter();
    writer.version(HLSVersion::_7);

    assert_eq!(writer.finish(), expected_manifest);
  }

  // STREAM INF
  #[test]
  fn test_stream_inf_with_minumum_options() {
    let expected_manifest = "#EXT-X-STREAM-INF:BANDWIDTH=100000\nhttps://domain.com/some/foo/bar/path.m3u8\n";

    let mut writer = HLSWriter::createWriter();
    writer.stream_inf(
      "https://domain.com/some/foo/bar/path.m3u8",
      100000,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
    );

    assert_eq!(writer.finish(), expected_manifest);
  }

  #[test]
  fn test_stream_inf_with_maximum_options() {
    let expected_manifest = "#EXT-X-STREAM-INF:\
                                  BANDWIDTH=100000,\
                                  AVERAGE-BANDWIDTH=50000,\
                                  FRAME-RATE=29.97,\
                                  HDCP-LEVEL=TYPE-0,\
                                  ALLOWED-CPC=\"com.example.drm1:SMART-TV/PC\",\
                                  RESOLUTION=1920x1080,\
                                  VIDEO-RANGE=SDR,\
                                  CODECS=\"avc1.42e00a,mp4a.40.2\",\
                                  AUDIO=\"a1\",\
                                  VIDEO=\"v1\",\
                                  SUBTITLES=\"sub1\",\
                                  CLOSED-CAPTIONS=\"cc1\"\
                                  \nhttps://domain.com/some/foo/bar/path.m3u8\n";

    let mut writer = HLSWriter::createWriter();
    writer.stream_inf(
      "https://domain.com/some/foo/bar/path.m3u8",
      100000,
      Option::Some(50000),
      Option::Some(29.97),
      Option::Some(HDCP_LEVEL::TYPE_0),
      Option::Some("com.example.drm1:SMART-TV/PC"),
      Option::Some(VideoResolution::_1080),
      Option::Some(VIDEO_RANGE::SDR),
      Option::Some("avc1.42e00a,mp4a.40.2"),
      Option::Some("a1"),
      Option::Some("v1"),
      Option::Some("sub1"),
      Option::Some("cc1"),
    );

    assert_eq!(writer.finish(), expected_manifest);
  }

  // MEDIA
  #[test]
  fn test_media_with_minumum_options() {
    let expected_manifest = "#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"a1\",NAME=\"English\"\n";

    let mut writer = HLSWriter::createWriter();
    writer.media(
      HLSMediaType::AUDIO,
      "a1",
      "English",
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None
    );

    assert_eq!(writer.finish(), expected_manifest);
  }

  #[test]
  fn test_media_with_maximum_options() {
    let expected_manifest = "#EXT-X-MEDIA:\
                                  TYPE=AUDIO,\
                                  GROUP-ID=\"a1\",\
                                  NAME=\"English\",\
                                  LANGUAGE=\"en-US\",\
                                  ASSOC-LANGUAGE=\"lang\",\
                                  DEFAULT=YES,\
                                  AUTO-SELECT=YES,\
                                  FORCED=NO,\
                                  INSTREAM-ID=\"CC1\",\
                                  CHARACTERISTICS=\"some,value\",\
                                  CHANNELS=\"2\",\
                                  URI=\"a1/prog_index.m3u8\"\n";

    let mut writer = HLSWriter::createWriter();
    writer.media(
      HLSMediaType::AUDIO,
      "a1",
      "English",
      Option::Some("a1/prog_index.m3u8"),
      Option::Some("en-US"),
      Option::Some("lang"),
      Option::Some(HLSBool::YES),
      Option::Some(HLSBool::YES),
      Option::Some(HLSBool::NO),
      Option::Some(CCInstreamId::CC1),
      Option::Some("some,value"),
      Option::Some("2")
    );

    assert_eq!(writer.finish(), expected_manifest);
  }

  // I FRAME STREAM INF
  #[test]
  fn test_i_frame_stream_inf_with_minumum_options() {
    let expected_manifest = "#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=100000,URI=\"https://domain.com/some/foo/bar/path.m3u8\"\n";

    let mut writer = HLSWriter::createWriter();
    writer.i_frame_stream_inf(
      "https://domain.com/some/foo/bar/path.m3u8",
      100000,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
      Option::None,
    );

    assert_eq!(writer.finish(), expected_manifest);
  }

  #[test]
  fn test_i_frame_stream_inf_with_maximum_options() {
    let expected_manifest = "#EXT-X-I-FRAME-STREAM-INF:\
                                  BANDWIDTH=100000,\
                                  AVERAGE-BANDWIDTH=50000,\
                                  HDCP-LEVEL=TYPE-0,\
                                  ALLOWED-CPC=\"com.example.drm1:SMART-TV/PC\",\
                                  RESOLUTION=1920x1080,\
                                  VIDEO-RANGE=SDR,\
                                  CODECS=\"avc1.42e00a\",\
                                  VIDEO=\"v1\",\
                                  URI=\"https://domain.com/some/foo/bar/path.m3u8\"\n";

    let mut writer = HLSWriter::createWriter();
    writer.i_frame_stream_inf(
      "https://domain.com/some/foo/bar/path.m3u8",
      100000,
      Option::Some(50000),
      Option::Some(HDCP_LEVEL::TYPE_0),
      Option::Some("com.example.drm1:SMART-TV/PC"),
      Option::Some(VideoResolution::_1080),
      Option::Some(VIDEO_RANGE::SDR),
      Option::Some("avc1.42e00a"),
      Option::Some("v1"),
    );

    assert_eq!(writer.finish(), expected_manifest);
  }

  // INDEPENDENT
  #[test]
  fn test_hls_independent() {
    let expected_manifest = "#EXT-X-INDEPENDENT-SEGMENTS\n";

    let mut writer = HLSWriter::createWriter();
    writer.independent();

    assert_eq!(writer.finish(), expected_manifest);
  }

  /**
   * Playlist Manifest Tags
   */

   //Soemthing
   #[test]
   fn test_something() {
     
   }
}