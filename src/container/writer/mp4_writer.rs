
pub struct Mp4Writer{
  data: Vec<u8>
}

impl Mp4Writer {

  pub fn create_mp4_writer() -> Mp4Writer {
    return Mp4Writer{
      data: vec![]
    }
  }

  pub fn duration(&mut self) -> &mut Mp4Writer {
    return self
  }

  pub fn pps(&mut self) -> &mut Mp4Writer {
    return self
  }

  pub fn sps(&mut self) -> &mut Mp4Writer {
    
    return self
  }

  pub fn build_init_segment() -> &'static [u8] {
    &[]
  }

  pub fn build_media_segment() -> &'static [u8] {
    &[]
  }
 }