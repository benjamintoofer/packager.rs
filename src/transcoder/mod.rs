pub mod ffmpeg;
pub mod bento;

pub enum VideoResolution {
  _1080_60,
  _1080_30,
  _720_60,
  _720_30,
  _480_30,
  _360_30,
}

impl VideoResolution {
  pub fn value(&self) -> String {
    match self {
        VideoResolution::_1080_60 => {"1920x1080".to_string()}
        VideoResolution::_1080_30 => {"1920x1080".to_string()}
        VideoResolution::_720_60 => {"1280x720".to_string()}
        VideoResolution::_720_30 => {"1280x720".to_string()}
        VideoResolution::_480_30 => {"854x480".to_string()}
        VideoResolution::_360_30 => {"640x360".to_string()}
    }
  }

  pub fn get_width(&self) -> u16 {
    match self {
        VideoResolution::_1080_60 => {1080}
        VideoResolution::_1080_30 => {1080}
        VideoResolution::_720_60 => {720}
        VideoResolution::_720_30 => {720}
        VideoResolution::_480_30 => {480}
        VideoResolution::_360_30 => {360}
    }
  }

  pub fn get_scale(&self) -> &str {
    match self {
        VideoResolution::_1080_60 => {"1920:1080"}
        VideoResolution::_1080_30 => {"1920:1080"}
        VideoResolution::_720_60 => {"1280:720"}
        VideoResolution::_720_30 => {"1280:720"}
        VideoResolution::_480_30 => {"854:480"}
        VideoResolution::_360_30 => {"640:360"}
    }
  }

  pub fn get_fps_str(&self) -> &str {
    match self {
        VideoResolution::_1080_60 => {"60000/1001"}
        VideoResolution::_1080_30 => {"30000/1001"}
        VideoResolution::_720_60 => {"60000/1001"}
        VideoResolution::_720_30 => {"30000/1001"}
        VideoResolution::_480_30 => {"30000/1001"}
        VideoResolution::_360_30 => {"30000/1001"}
    }
  }

  pub fn get_fps(&self) -> u8 {
    match self {
        VideoResolution::_1080_60 => {60}
        VideoResolution::_1080_30 => {30}
        VideoResolution::_720_60 => {30}
        VideoResolution::_720_30 => {30}
        VideoResolution::_480_30 => {30}
        VideoResolution::_360_30 => {30}
    }
  }
}

pub enum AudioSampleRates {
  _96k,
  _48k,
}

impl AudioSampleRates {
  pub fn value(&self) -> &str {
    match self {
        AudioSampleRates::_96k => {"96000"}
        AudioSampleRates::_48k => {"48000"}
    }
  }
}