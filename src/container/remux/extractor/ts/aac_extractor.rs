use crate::{container::{isobmff::{descriptors::{aac_audio_specific_config::AACAudioSpecificConfigBuilder, dec_config_descriptor::DecoderConfigDescriptorBuilder, es_descriptor::ESDescriptorBuidler}, sample_entry::{audio_sample_entry::AudioSampleEntryBuilder, mp4a_sample_entry::MP4ASampleEntryBuilder, sample_entry::SampleEntryBuilder}, HandlerType}, remux::{extractor::TSExtractor, map_sample_frequency_index}, transport_stream::{adts::ADTSFrame, pes_packet, adts::ADTS}, writer::mp4_writer::{SampleInfo, Mp4Writer}}, error::CustomError};
use crate::container::isobmff::BoxBuilder;

pub struct AACExtractor {
  bucket: Vec<u8>,
  current_pts: u64,
  current_dts: u64,
  adts_frames: Vec<ADTSFrame>,
  sample_frequency_index: Option<u8>
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
    }

    if let Some(pts) = pes.pts {
      let dts = pes.dts.map_or_else(||pts, |dts|dts);
      self.current_dts = dts;
      self.current_pts = pts;
    }

    self.bucket.append(&mut pes.payload_data.to_vec());
    Ok(())
  }

  fn is_all_same_timestamps(&self) -> bool {
    true
  }

  fn is_signed_comp_offset(&self) -> bool {
    false
  }

  fn build_sample_entry(&mut self) -> Result<Vec<u8>, CustomError> {
    if self.adts_frames.len() > 0 {
      let frame = &self.adts_frames[0];
      return MP4ASampleEntryBuilder::create_builder()
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
        .build();
    }
    println!("AACExtractor :: build_sample_entry :: No ADTS frames available. Returning empty vector");
    Ok(vec![])
  }

  fn flush_final_media(&mut self) -> Result<(), CustomError> {
    let mut adts_frames = ADTS::parse(&self.bucket)?
      .iter_mut()
      .map(|frame|{
        // If the sample_frequency_index has not been set yet, we will store it to be used to determine the timescale for later.
        self.sample_frequency_index = Some(frame.header.sampling_frequency_index);
        frame.set_pts(self.current_pts);
        frame.set_dts(self.current_dts);
        return std::mem::take(frame)
      })
      .collect();
    println!("ADTS FRAMES {}", self.adts_frames.len());
    self.adts_frames.append(&mut adts_frames);

    Ok(())
  }

  fn get_timescale(&self) -> u32 {
    self.sample_frequency_index
      .and_then(|x|Some(map_sample_frequency_index(x)))
      .unwrap_or_default()
  }

  fn get_init_segment(&mut self) -> Result<Vec<u8>, CustomError> {
    let sample_entry_data = self.build_sample_entry()?;
    let track_id = 2usize;

    Mp4Writer::create_mp4_writer()
      .timescale(self.get_timescale())
      .handler(HandlerType::SOUN)
      .track_id(track_id)
      .build_init_segment(sample_entry_data)
  }

  fn get_media_segment(&mut self) -> Result<Vec<u8>, CustomError> {
    let media_data = AACExtractor::convert_adts_frame_to_sample_infos(std::mem::take(&mut self.adts_frames));
    let track_id = 2usize;
    Mp4Writer::create_mp4_writer()
      .track_id(track_id)
      .timescale(self.get_timescale())
      .default_sample_duration(self.get_default_sample_duration())
      .samples(media_data)
      .build_media_segment()
  }

  fn get_default_sample_duration(&self) -> u32 {
    return 1024
  }
}

impl AACExtractor {
  pub fn create() -> AACExtractor {
    AACExtractor {
      bucket: vec![],
      adts_frames: vec![],
      current_pts: 0,
      current_dts: 0,
      sample_frequency_index: None,
    }
  }

  fn convert_adts_frame_to_sample_infos(adts_frames: Vec<ADTSFrame>) -> Vec<SampleInfo> {
    let sample_infos: Vec<SampleInfo> = adts_frames
      .iter() 
      .map(|af| {
        // Create the sample data
        return SampleInfo{
          sample_flags: None, // Nothing for now. Determine later if this needs to be set
          data: af.data.to_owned(),
          dts: af.dts,
          pts: af.pts,
        }
      })
      .collect();
    sample_infos
  }
}