use crate::transcoder::VideoResolution;
use crate::manifest::hls::HLSVersion;
use crate::manifest::hls::HLSMediaType;

static EXT_TAG_PREFIX: &'static str = "#EXT";

pub enum HLSBool {
  YES,
  NO
}
impl HLSBool {
  pub fn value(&self) -> &str {
    match self {
        HLSBool::YES => {"YES"}
        HLSBool::NO => {"NO"}
    }
  }
}

pub enum CCInstreamId {
  CC1,
  CC2,
  CC3,
  CC4,
  NONE
}

impl CCInstreamId {
  pub fn value(&self) -> &str {
    match self {
        CCInstreamId::CC1 => {"CC1"}
        CCInstreamId::CC2 => {"CC2"}
        CCInstreamId::CC3 => {"CC3"}
        CCInstreamId::CC4 => {"CC4"}
        CCInstreamId::NONE => {"NONE"}
    }
  }
}

pub struct HLSWriter {
  hls_manifest_str: String
}


impl HLSWriter {
  pub fn start_hls(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}M3U\n", EXT_TAG_PREFIX).as_str());
    self
  }

  fn version(&mut self, version: HLSVersion) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-VERSION:{}\n", EXT_TAG_PREFIX, version.value()).as_str());
    self
  }

  fn stream_inf(
    &mut self,
    path: &str,
    bandwidth: u32,
    average_bandwidth: Option<u32>,
    frame_rate: Option<u32>,
    hdcp_level: Option<u32>,              // TODO (benjamintoofer@gmail.com): Enumerated value (https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-07#section-4.4.6.2)
    allowed_cpc: Option<&str>,
    resolution: Option<VideoResolution>,
    video_range: Option<String>,          // TODO (benjamintoofer@gmail.com): Enumerated value (https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-07#section-4.4.6.2)
    codecs: Option<String>,
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
      self.hls_manifest_str.push_str(format!(",HDCP-LEVEL={}", hdcp_level).as_str());  
    }
    if let Some(allowed_cpc) = allowed_cpc {
      self.hls_manifest_str.push_str(format!(",ALLOWED-CPC={}", allowed_cpc).as_str());  
    }
    if let Some(resoltuion) = resolution {
      self.hls_manifest_str.push_str(format!(",RESOLUTION={}", resoltuion.value()).as_str());  
    }
    if let Some(video_range) = video_range {
      self.hls_manifest_str.push_str(format!(",VIDEO-RANGE={}", video_range).as_str());  
    }
    if let Some(codecs) = codecs {
      self.hls_manifest_str.push_str(format!(",CODECS={}", codecs).as_str());  
    }
    // Groupd ids
    if let Some(audio) = audio {
      self.hls_manifest_str.push_str(format!(",AUDIO={}", audio).as_str());  
    }
    if let Some(video) = video {
      self.hls_manifest_str.push_str(format!(",VIDEO={}", video).as_str());  
    }
    if let Some(subtitles) = subtitles {
      self.hls_manifest_str.push_str(format!(",SUBTITLES={}", subtitles).as_str());  
    }
    if let Some(closed_captions) = closed_captions {
      self.hls_manifest_str.push_str(format!(",CLOSED-CAPTIONS={}", closed_captions).as_str());  
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
    self.hls_manifest_str.push_str(format!("{}-X-MEDIA:TYPE={},NAME={},GROUP-ID={}", EXT_TAG_PREFIX, media_type.value(), name, group_id).as_str());  
    if let Some(language) = language {
      self.hls_manifest_str.push_str(format!(",LANGUAGE={}", language).as_str());  
    }
    if let Some(assoc_language) = assoc_language {
      self.hls_manifest_str.push_str(format!(",ASSOC-LANGUAGE={}", assoc_language).as_str());  
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
      self.hls_manifest_str.push_str(format!(",INSTREAM-ID={}", instream_id.value()).as_str());  
    }
    if let Some(characteristics) = characteristics {
      self.hls_manifest_str.push_str(format!(",CHARACTERISTICS={}", characteristics).as_str());  
    }
    if let Some(channels) = channels {
      self.hls_manifest_str.push_str(format!(",CHANNELS={}", channels).as_str());  
    }
    if let Some(uri) = uri {
      self.hls_manifest_str.push_str(format!(",URI={}", uri).as_str());  
    }
    self.hls_manifest_str.push('\n');
    self
  }

  fn i_frame_stream_inf(
    &mut self,
    uri: &str,
    bandwidth: u32,
    average_bandwidth: Option<u32>,
    hdcp_level: Option<u32>,              // TODO (benjamintoofer@gmail.com): Enumerated value (https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-07#section-4.4.6.2)
    allowed_cpc: Option<&str>,
    resolution: Option<VideoResolution>,
    video_range: Option<String>,          // TODO (benjamintoofer@gmail.com): Enumerated value (https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-07#section-4.4.6.2)
    codecs: Option<String>,
    // group-ids
    video: Option<&str>,
  ) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-I-FRAME-STREAM-INF:BANDWIDTH={}", EXT_TAG_PREFIX, bandwidth).as_str());  
    if let Some(average_bandwidth) = average_bandwidth {
      self.hls_manifest_str.push_str(format!(",AVERAGE-BANDWIDTH={}", average_bandwidth).as_str());  
    }
    if let Some(hdcp_level) = hdcp_level {
      self.hls_manifest_str.push_str(format!(",HDCP-LEVEL={}", hdcp_level).as_str());  
    }
    if let Some(allowed_cpc) = allowed_cpc {
      self.hls_manifest_str.push_str(format!(",ALLOWED-CPC={}", allowed_cpc).as_str());  
    }
    if let Some(resoltuion) = resolution {
      self.hls_manifest_str.push_str(format!(",RESOLUTION={}", resoltuion.value()).as_str());  
    }
    if let Some(video_range) = video_range {
      self.hls_manifest_str.push_str(format!(",VIDEO-RANGE={}", video_range).as_str());  
    }
    if let Some(codecs) = codecs {
      self.hls_manifest_str.push_str(format!(",CODECS={}", codecs).as_str());  
    }
    // Group ids
    if let Some(video) = video {
      self.hls_manifest_str.push_str(format!(",VIDEO={}", video).as_str());  
    }
    self.hls_manifest_str.push_str(format!(",URI={}\n", uri).as_str());  
    self
  }

  fn independent(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push_str(format!("{}-X-INDEPENDENT-SEGMENTS\n", EXT_TAG_PREFIX).as_str());  
    self
  }

  fn new_line(&mut self) -> &HLSWriter {
    self.hls_manifest_str.push('\n'); 
    self
  }

  fn end(&self) -> &str {
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

fn hls_template() -> String {
  let manifest = String::from(EXT_TAG_PREFIX);

  manifest
}