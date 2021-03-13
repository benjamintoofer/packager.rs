pub mod ffmpeg;
pub mod bento;

pub enum VideoResolution {
  _1080,
  _720,
  _480,
  _360
}

impl VideoResolution {
  pub fn value(&self) -> String {
    match self {
        VideoResolution::_1080 => {"1920x1080".to_string()}
        VideoResolution::_720 => {"1280x720".to_string()}
        VideoResolution::_480 => {"854x480".to_string()}
        VideoResolution::_360 => {"640x360".to_string()}
    }
  }
}