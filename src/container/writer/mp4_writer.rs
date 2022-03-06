use crate::{codec::{Codec, h264::sequence_parameter_set::SequenceParameterSet}, container::{isobmff::{BoxBuilder, boxes::{ftyp::FTYPBuilder, hdlr::HDLRBuilder, mdat::MDATBuilder, mdhd::MDHDBuilder, mdia::MDIABuilder, minf::MINFBuilder, moof::MOOFBuilder, moov::MOOVBuilder, mvex::MVEXBuilder, mvhd::MVHDBuilder, stbl::STBLBuilder, stsd::STSDBuilder, tfdt::TFDTBuilder, tfhd::TFHDBuilder, tkhd::TKHDBuilder, traf::TRAFBuilder, trak::TRAKBuilder, trex::TREXBuilder, trun::TRUNBuilder, vmhd::VMHDBuilder}, configuration_records::avcC::AVCDecoderConfigurationRecordBuilder, sample_entry::{avc_sample_entry::AVCSampleEntryBuilder, sample_entry::SampleEntryBuilder, visual_sample_entry::VisualSampleEntryBuilder}}, transport_stream::adts::{ADTSFrame, ADTSHeader}}, error::CustomError};
use crate::container::isobmff::HandlerType;
use crate::container::isobmff::nal::NalRep;

pub struct Mp4Writer{
  sps: Vec<u8>,
  pps: Vec<u8>,
  media_nals: Vec<NalRep>,
  adts_frames: Vec<ADTSFrame>,
  timescale: usize,
}

impl Mp4Writer {

  pub fn create_mp4_writer() -> Mp4Writer {
    return Mp4Writer{
      sps: vec![],
      pps: vec![],
      media_nals: vec![],
      timescale: 0,
      adts_frames: vec![],
    }
  }
}

impl Mp4Writer {
  
  pub fn timescale(mut self, timescale: usize) -> Mp4Writer {
    self.timescale = timescale;
    self
  }

  // pub fn sample_entry(mut self, impl BoxBuilder) -> Mp4Writer {
  //   self.
  // }

  pub fn pps(mut self, pps: &[u8]) -> Mp4Writer {
    self.pps = pps.to_vec();
    self
  }

  pub fn sps(mut self, sps: &[u8]) -> Mp4Writer {
    self.sps = sps.to_vec();
    self
  }

  pub fn nals(mut self, media_nals: Vec<NalRep>) -> Mp4Writer {
    self.media_nals = media_nals;
    self
  }

  pub fn build_init_segment(self, sample_entry: impl BoxBuilder + 'static) -> Result<Vec<u8>, CustomError> {
    let sps = SequenceParameterSet::parse(&self.sps)?;
    Ok([
      FTYPBuilder::create_builder().build(),
      MOOVBuilder::create_builder()
        .mvhd(
          MVHDBuilder::create_builder()
            .timescale(self.timescale)
        )
        .trak(
          TRAKBuilder::create_builder()
            .tkhd(
              TKHDBuilder::create_builder()
                .track_id(1) // CHANGE THIS
                .width(sps.width())
                .height(sps.height())
            )
            .mdia(
              MDIABuilder::create_builder()
                .mdhd(
                  MDHDBuilder::create_builder()
                    .timescale(self.timescale)
                )
                .hdlr(
                  HDLRBuilder::create_builder()
                    .handler_type(HandlerType::VIDE) //CHANGE THIS
                )
                .minf(
                  MINFBuilder::create_builder()
                    .media_header(Box::new(VMHDBuilder::create_builder()))
                    .stbl(
                      STBLBuilder::create_builder()
                        .stsd(
                          STSDBuilder::create_builder()
                            .sample_entry(
                              Box::new(
                                // AVCSampleEntryBuilder::create_builder()
                                //   .sample_entry(
                                //     SampleEntryBuilder::create_builder()
                                //   )
                                //   .visual_sample_entry(
                                //     VisualSampleEntryBuilder::create_builder()
                                //       .sps(&self.sps)
                                //   )
                                //   .avc_c(
                                //     AVCDecoderConfigurationRecordBuilder::create_builder()
                                //       .sps(&self.sps)
                                //       .pps(&self.pps)
                                //   )
                                sample_entry
                              )
                          )
                        )
                    )
                )
            )
        )
        .mvex(
          MVEXBuilder::create_builder()
            .trex(
              TREXBuilder::create_builder()
                .track_id(1) // CHANGE THIS
                .default_sample_size(0)// CHANGE THIS
                .default_sample_duration(0) // CHANGE THIS
                .default_sample_flags(0) // CHANGE THIS
            )
        )
        .build()?
    ].concat())
  }
  pub fn build_aac_media_segment(self) -> Result<Vec<u8>, CustomError> {
    Ok([
      MOOFBuilder::create_builder()
        .traf(
          TRAFBuilder::create_builder()
            .tfhd(
              TFHDBuilder::create_builder()
                .sample_duration(3000) // CHANGE THIS
                .track_id(1) // CHANGE THIS
            )
            .tfdt(
              TFDTBuilder::create_builder()
                .base_media_decode_time(self.media_nals[0].dts as usize)
            )
            .trun(
              TRUNBuilder::create_builder()
                .version(0)
                .flags(0x0205)
                .first_sample_flags(0x2000000)
                .samples(self.media_nals.clone())
            )
        )
        .build()?,
      MDATBuilder::create_builder()
        .media_data(MDATBuilder::convert_adts_frames(self.adts_frames))
        .build()?
    ].concat())
  }
  pub fn build_media_segment(self) -> Result<Vec<u8>, CustomError> {
    // println!("LOWEST DTS: {}", self.media_nals[0].dts);
    Ok([
      MOOFBuilder::create_builder()
        .traf(
          TRAFBuilder::create_builder()
            .tfhd(
              TFHDBuilder::create_builder()
                .sample_duration(3000) // CHANGE THIS
                .track_id(1) // CHANGE THIS
            )
            .tfdt(
              TFDTBuilder::create_builder()
                .base_media_decode_time(self.media_nals[0].dts as usize)
            )
            .trun(
              TRUNBuilder::create_builder()
                .version(0)
                .flags(0x0205)
                .first_sample_flags(0x2000000)
                .samples(self.media_nals.clone())
            )
        )
        .build()?,
      MDATBuilder::create_builder()
        .media_data(MDATBuilder::convert_nal_units(self.media_nals))
        .build()?
    ].concat())
  }
 }

 // ffmpeg -i ~/Desktop/seg_2_complete_v.ts -video_track_timescale 90000 ~/Desktop/seg_2_complete_v.mp4
