use crate::{container::{isobmff::{boxes::{ftyp::FTYPBuilder, hdlr::HDLRBuilder, mdat::MDATBuilder, mdhd::MDHDBuilder, mdia::MDIABuilder, minf::MINFBuilder, moof::MOOFBuilder, moov::MOOVBuilder, mvex::MVEXBuilder, mvhd::MVHDBuilder, stbl::STBLBuilder, stsd::STSDBuilder, tfdt::TFDTBuilder, tfhd::TFHDBuilder, tkhd::TKHDBuilder, traf::TRAFBuilder, trak::TRAKBuilder, trex::TREXBuilder, trun::TRUNBuilder, vmhd::VMHDBuilder, smhd::SMHDBuilder}}}, error::CustomError};
use crate::container::isobmff::HandlerType;
use crate::error::{construct_error, error_code::{MajorCode, TransportStreamMinorCode}};
use crate::container::isobmff::BoxBuilder;

#[derive(Clone)]
pub struct SampleInfo {
  pub dts: u64,
  pub pts: u64,
  pub sample_flags: Option<u32>,
  pub sample_duration: Option<u32>,
  pub data: Vec<u8>,
}

pub struct Mp4Writer{
  samples: Vec<SampleInfo>,
  width: usize,
  height: usize,
  timescale: u32,
  track_id: usize,
  trun_version: u8,
  is_all_same_timestamps: bool,
  default_sample_duration: Option<u32>,
  handler_type: Option<HandlerType>
}

impl Mp4Writer {

  pub fn create_mp4_writer() -> Mp4Writer {
    return Mp4Writer{
      timescale: 0,
      width: 0,
      height: 0,
      trun_version: 0,
      is_all_same_timestamps: true,
      default_sample_duration: None,
      track_id: 1,
      samples: vec![],
      handler_type: None
    }
  }
}

impl Mp4Writer {
  
  pub fn timescale(mut self, timescale: u32) -> Mp4Writer {
    self.timescale = timescale;
    self
  }

  pub fn samples(mut self, samples: Vec<SampleInfo>) -> Mp4Writer {
    self.samples =  samples;
    self
  }

  pub fn width(mut self, width: usize) -> Mp4Writer {
    self.width = width;
    self
  }

  pub fn height(mut self, height: usize) -> Mp4Writer {
    self.height = height;
    self
  }

  pub fn track_id(mut self, track_id: usize) -> Mp4Writer {
    self.track_id = track_id;
    self
  }

  pub fn default_sample_duration(mut self, default_sample_duration: u32) -> Mp4Writer {
    self.default_sample_duration = Some(default_sample_duration);
    self
  }

  pub fn handler(mut self, handler_type: HandlerType) -> Mp4Writer {
    self.handler_type = Some(handler_type);
    self
  }

  pub fn trun_version(mut self, version: u8) -> Mp4Writer {
    self.trun_version = version;
    self
  }

  pub fn is_all_same_timestamps(mut self, same: bool) -> Mp4Writer {
    self.is_all_same_timestamps = same;
    self
  }

  pub fn build_init_segment(self, sample_entry: Vec<u8>) -> Result<Vec<u8>, CustomError> {
    let handler_type = self.handler_type.ok_or_else(||construct_error(
      MajorCode::REMUX,
      Box::new(TransportStreamMinorCode::PARSE_TS_ERROR),
      "Handler type not set".to_string(),
      file!(),
      line!()))?;
    let media_header: Box<dyn BoxBuilder> = match handler_type {
      HandlerType::VIDE => Box::new(VMHDBuilder::create_builder()),
      HandlerType::SOUN => Box::new(SMHDBuilder::create_builder()),
      _ => Box::new(VMHDBuilder::create_builder())
    };

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
                .track_id(self.track_id) 
                .width(self.width)
                .height(self.height)
            )
            .mdia(
              MDIABuilder::create_builder()
                .mdhd(
                  MDHDBuilder::create_builder()
                    .timescale(self.timescale)
                )
                .hdlr(
                  HDLRBuilder::create_builder()
                    .handler_type(handler_type) //CHANGE THIS
                )
                .minf(
                  MINFBuilder::create_builder()
                    .media_header(media_header)
                    .stbl(
                      STBLBuilder::create_builder()
                        .stsd(
                          STSDBuilder::create_builder()
                            .sample_entry(
                              sample_entry
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
                .track_id(self.track_id)
                .default_sample_size(0)// CHANGE THIS
                .default_sample_duration(0) // CHANGE THIS
                .default_sample_flags(0) // CHANGE THIS
            )
        )
        .build()?
    ].concat())
  }

  pub fn build_media_segment(self) -> Result<Vec<u8>, CustomError> {


    Ok([
      MOOFBuilder::create_builder()
        .traf(
          TRAFBuilder::create_builder()
            .tfhd(
              TFHDBuilder::create_builder()
                .sample_duration(self.default_sample_duration)
                .sample_description_index(1)
                .sample_flags(0x01010000) // CHANGE THIS
                .track_id(self.track_id) 
            )
            .tfdt(
              TFDTBuilder::create_builder()
                .base_media_decode_time(self.samples[0].dts as usize)
            )
            .trun(
              TRUNBuilder::create_builder()
                .version(self.trun_version as usize)
                // .flags(0x0205) // CHANGE THIS
                .sample_composition_time_offsets_present(!self.is_all_same_timestamps)
                .first_sample_flags(0x2000000) // CHANGE THIS
                .samples(self.samples.clone())
            )
        )
        .build()?,
      MDATBuilder::create_builder()
        .media_data(MDATBuilder::merge_samples(self.samples))
        .build()?
    ].concat())
  }
 }

 // ffmpeg -i ~/Desktop/seg_2_complete_v.ts -video_track_timescale 90000 ~/Desktop/seg_2_complete_v.mp4
