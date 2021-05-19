pub mod hls_generator;
pub mod hls_writer;

pub enum HLSVersion {
  _4,
  _7,
  // TODO (benjamintoofer@gmail.com): Add version 8
}

impl HLSVersion {
  pub fn value(&self) -> u8 {
    match self {
        HLSVersion::_4 => {4u8}
        HLSVersion::_7 => {7u8}
    }
  }
}

pub enum HLSPlaylistType {
  EVENT,
  VOD
}
impl HLSPlaylistType {
  pub fn value(&self) -> &str {
    match self {
        HLSPlaylistType::EVENT => {"EVENT"}
        HLSPlaylistType::VOD => {"VOD"}
    }
  }
}

#[allow(non_camel_case_types)]
pub enum HLSMediaType {
  AUDIO,
  VIDEO,
  CLOSED_CAPTIONS,
  SUBTITLES
}

impl HLSMediaType {
  pub fn value(&self) -> &str {
    match self {
        HLSMediaType::AUDIO => {"AUDIO"}
        HLSMediaType::VIDEO => {"VIDEO"}
        HLSMediaType::CLOSED_CAPTIONS => {"CLOSED-CAPTIONS"}
        HLSMediaType::SUBTITLES => {"SUBTITLES"}
    }
  }
}

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

#[allow(non_camel_case_types)]
pub enum HDCP_LEVEL {
  TYPE_0,
  TYPE_1,
  NONE
}

impl HDCP_LEVEL {
  pub fn value(&self) -> &str {
    match self {
        HDCP_LEVEL::TYPE_0 => {"TYPE-0"}
        HDCP_LEVEL::TYPE_1 => {"TYPE-1"}
        HDCP_LEVEL::NONE => {"NONE"}
    }
  }
}

pub enum VIDEO_RANGE {
  SDR,
  HLG,
  PQ
}

impl VIDEO_RANGE {
  pub fn value(&self) -> &str {
    match self {
        VIDEO_RANGE::SDR => {"SDR"}
        VIDEO_RANGE::HLG => {"HLG"}
        VIDEO_RANGE::PQ => {"PQ"}
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
