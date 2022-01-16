use crate::{codec::h264::sequence_parameter_set::SequenceParameterSet, container::isobmff::{boxes::{ftyp::FTYPBuilder, hdlr::HDLRBuilder, mdhd::MDHDBuilder, mdia::MDIABuilder, minf::MINFBuilder, moov::MOOVBuilder, mvex::MVEXBuilder, mvhd::MVHDBuilder, stbl::STBLBuilder, stsd::STSDBuilder, tkhd::TKHDBuilder, trak::TRAKBuilder, trex::TREXBuilder, vmhd::VMHDBuilder}, configuration_records::avcC::AVCDecoderConfigurationRecordBuilder, sample_entry::{avc_sample_entry::AVCSampleEntryBuilder, sample_entry::SampleEntryBuilder, visual_sample_entry::VisualSampleEntryBuilder}}, error::CustomError};
use crate::container::isobmff::HandlerType;


pub struct Mp4Writer{
  sps: Vec<u8>,
  pps: Vec<u8>,
  timescale: usize,
}

impl Mp4Writer {

  pub fn create_mp4_writer() -> Mp4Writer {
    return Mp4Writer{
      sps: vec![],
      pps: vec![],
      timescale: 0,
    }
  }
  
  pub fn timescale(mut self, timescale: usize) -> Mp4Writer {
    self.timescale = timescale;
    self
  }

  pub fn pps(mut self, pps: &[u8]) -> Mp4Writer {
    self.pps = pps.to_vec();
    self
  }

  pub fn sps(mut self, sps: &[u8]) -> Mp4Writer {
    self.sps = sps.to_vec();
    self
  }

  pub fn build_init_segment(self) -> Result<Vec<u8>, CustomError>{
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
                                AVCSampleEntryBuilder::create_builder()
                                  .sample_entry(
                                    SampleEntryBuilder::create_builder()
                                  )
                                  .visual_sample_entry(
                                    VisualSampleEntryBuilder::create_builder()
                                      .sps(&self.sps)
                                  )
                                  .avc_c(
                                    AVCDecoderConfigurationRecordBuilder::create_builder()
                                      .sps(&self.sps)
                                      .pps(&self.pps)
                                  )
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

  pub fn build_media_segment() -> &'static [u8] {
    &[]
  }
 }