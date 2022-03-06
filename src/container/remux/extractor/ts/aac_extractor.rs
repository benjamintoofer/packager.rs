use crate::{container::transport_stream::{adts::ADTSFrame, pes_packet, adts::ADTS}, error::CustomError};



pub struct AACExtractor <IF, MF>
where
  IF: Fn(&ADTSFrame),
  MF: Fn(&Vec<ADTSFrame>),
{
  bucket: Vec<u8>,
  current_pts: u64,
  current_dts: u64,
  adts_frames: Vec<ADTSFrame>,
  init_callback: Option<IF>,
  media_callback: Option<MF>,
}

impl<IF, MF> AACExtractor<IF, MF>
where
  IF: Fn(&ADTSFrame),
  MF: Fn(&Vec<ADTSFrame>),
{
  pub fn create() -> AACExtractor<IF, MF> {
    AACExtractor {
      bucket: vec![],
      adts_frames: vec![],
      current_pts: 0,
      current_dts: 0,
      init_callback: None,
      media_callback: None,
    }
  }

  pub fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) -> Result<(), CustomError> {

    // Flush bucket since we are encountering a new ADTS sequence
    if pes.pts.is_some() && !self.bucket.is_empty() {
      let adts_packet = self.bucket.clone();
      self.bucket.clear();

     let mut adts_frames = ADTS::parse(&adts_packet)?;
     self.adts_frames.append(&mut adts_frames);

     // If we have am aac frame, we can immediatley begin generating the init segment

    }

    if let Some(pts) = pes.pts {
      let dts = pes.dts.map_or_else(||pts, |dts|dts);
      self.current_dts = dts;
      self.current_pts = pts;
    }

    self.bucket.append(&mut pes.payload_data.to_vec());

    Ok(())
  }

  fn flush_final_media(&mut self) -> Result<(), CustomError> {
    let mut adts_frames = ADTS::parse(&self.bucket)?;
    self.adts_frames.append(&mut adts_frames);
    Ok(())
  }

  fn listen_for_init_data(&mut self, callback: IF) -> &Self {
      self.init_callback = Some(callback);
      return self;
  }

  fn listen_for_media_data(&mut self, callback: MF) -> &Self {
      self.media_callback = Some(callback);
      return self;
  }
}