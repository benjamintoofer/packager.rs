use crate::{container::{isobmff::{descriptors::{aac_audio_specific_config::AACAudioSpecificConfigBuilder, dec_config_descriptor::DecoderConfigDescriptorBuilder, es_descriptor::ESDescriptorBuidler}, sample_entry::{audio_sample_entry::AudioSampleEntryBuilder, mp4a_sample_entry::MP4ASampleEntryBuilder, sample_entry::SampleEntryBuilder}}, remux::{extractor::TSExtractor, map_sample_frequency_index}, transport_stream::{adts::ADTSFrame, pes_packet, adts::ADTS}, writer::mp4_writer::SampleInfo}, error::CustomError};
use crate::container::isobmff::BoxBuilder;

pub struct AACExtractor {
  bucket: Vec<u8>,
  current_pts: u64,
  current_dts: u64,
  adts_frames: Vec<ADTSFrame>,
  init_callback: Option<fn(Vec<u8>)>,
  media_callback: Option<fn(Vec<SampleInfo>)>,
  mp4a_sample_entry_is_set: bool
}

impl TSExtractor for AACExtractor {
  fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) -> Result<(), CustomError> {
    // Flush bucket since we are encountering a new ADTS sequence
    if pes.pts.is_some() && !self.bucket.is_empty() {
      let adts_packet = self.bucket.clone();
      self.bucket.clear();
      let mut adts_frames: Vec<ADTSFrame> = ADTS::parse(&adts_packet)?
        .iter_mut()
        .map(|frame|{
          frame.set_pts(self.current_pts);
          frame.set_dts(self.current_dts);
          return std::mem::take(frame)
        })
        .collect();
      self.adts_frames.append(&mut adts_frames);

      // If we have an aac frame, we can immediatley begin generating the init segment
      if !self.mp4a_sample_entry_is_set && self.adts_frames.len() > 0 {
        if let Some(cb) = &self.init_callback {
          let frame = &self.adts_frames[0];
          let sample_entry = MP4ASampleEntryBuilder::create_builder()
            .sample_entry(
              SampleEntryBuilder::create_builder()
            )
            .audio_sample_entry(
              AudioSampleEntryBuilder::create_builder()
                .channel_count(frame.header.channel_configuration.into())
                .sample_rate(map_sample_frequency_index(frame.header.sampling_frequency_index))
            )
            .esds(
              ESDescriptorBuidler::create_builder()
                .dec_conf_desc(
                  DecoderConfigDescriptorBuilder::create_builder()
                    .aac_audio_specific_config(
                      AACAudioSpecificConfigBuilder::create_builder()
                        .channel_count(frame.header.channel_configuration.into())
                        .sampling_frequency_index(frame.header.sampling_frequency_index.into())
                    )
                )
            )
            .build()?;
          cb(sample_entry);
        }
        self.mp4a_sample_entry_is_set = true;
      }
    }

    if let Some(pts) = pes.pts {
      let dts = pes.dts.map_or_else(||pts, |dts|dts);
      self.current_dts = dts;
      self.current_pts = pts;
    }

    self.bucket.append(&mut pes.payload_data.to_vec());
    Ok(())
  }

  fn is_all_same_timestamps(self) -> bool {
    true
  }

  fn is_signed_comp_offset(self) -> bool {
    false
  }

  fn build_sample_entry(self) -> Vec<u8> {
    todo!()
  }

  fn flush_final_media(&mut self) -> Result<(), CustomError> {
    let mut adts_frames = ADTS::parse(&self.bucket)?
      .iter_mut()
      .map(|frame|{
        frame.set_pts(self.current_pts);
        frame.set_dts(self.current_dts);
        return std::mem::take(frame)
      })
      .collect();
    println!("ADTS FRAMES {}", self.adts_frames.len());
    self.adts_frames.append(&mut adts_frames);

    if let Some(cb) = &self.media_callback {
      cb(AACExtractor::convert_adts_frame_to_sample_infos(std::mem::take(&mut self.adts_frames)));
    }
    Ok(())
  }

  fn listen_for_init_data(&mut self, callback: fn(Vec<u8>)) {
    self.init_callback = Some(callback);
  }

  fn listen_for_media_data(&mut self, callback: fn(Vec<SampleInfo>)) {
    self.media_callback = Some(callback);
  }
}

impl AACExtractor {
  pub fn create() -> AACExtractor {
    AACExtractor {
      bucket: vec![],
      adts_frames: vec![],
      current_pts: 0,
      current_dts: 0,
      init_callback: None,
      media_callback: None,
      mp4a_sample_entry_is_set: false,
    }
  }

  fn convert_adts_frame_to_sample_infos(adts_frames: Vec<ADTSFrame>) -> Vec<SampleInfo> {
    let sample_infos: Vec<SampleInfo> = adts_frames
      .iter() 
      .map(|af| {
        // Create the sample data
        return SampleInfo{
          data: af.data.to_owned(),
          dts: af.dts,
          pts: af.pts,
        }
      })
      .collect();
    sample_infos
  }
}