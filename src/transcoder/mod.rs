pub mod ffmpeg;
pub mod bento;

pub enum VideoSize {
  _1080,
  _720,
  _480,
  _360
}

impl VideoSize {
  pub fn value(&self) -> String {
    match self {
        VideoSize::_1080 => {"1920x1080".to_string()}
        VideoSize::_720 => {"1280x720".to_string()}
        VideoSize::_480 => {"854x480".to_string()}
        VideoSize::_360 => {"640x360".to_string()}
    }
  }
}