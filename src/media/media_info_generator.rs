use super::{MediaInfo, SegmentInfo, TrackInfo, TrackType};

use crate::{error::CustomError, isobmff::{HandlerType, boxes::{SampleFlag, hdlr::HDLR, iso_box::{find_box, get_box, get_init_segment_end}, sidx::{self, SIDX, SIDXReference}, stsd::STSD, tkhd::TKHDReader, trun::TRUN}, configuration_records::avcC, get_codec, sample_entry::avc_sample_entry::{self, AVCSampleEntry}}};


pub struct MediaInfoGenerator;

impl MediaInfoGenerator {
  pub fn temp(mp4: &[u8]) -> Result<SIDX, CustomError> {
    let mut offset = get_init_segment_end(&mp4);
    let sidx_box = SIDX::parse(&mp4)?;
    let mut tkhd_reader = TKHDReader::parse(&mp4)?;
    let hdlr = HDLR::parse(&mp4)?;
    let timescale = sidx_box.get_timescale();
    let references = sidx_box.get_references();
    let mut pts = sidx_box.get_earliest_presentation_time();

    /**
      Getting Track info
    */
    let track_id = tkhd_reader.get_track_id()?;
    let track_type = TrackType::handler_to_track_type(hdlr.get_handler_type());
    let group_id ="something";
    let codec = get_codec(track_type, &mp4)?;

    /**
      Getting Segment info
    */
    for sr in references {
      if sr.reference_type == false { // Skip reference types that are segment indexes (1)
          let duration: f32 = sr.subsegment_duration as f32 / timescale as f32;
          let mut start_with_i_frame = MediaInfoGenerator::determine_start_with_i_frame_with_sap(sr);
          if !start_with_i_frame {
            // If we cannot determine that the fragment starts with an iframe we will need to look into the fragment's
            // trun to determine the first_sample_flags (if available)
            let trun = find_box("moof", offset, mp4)
              .map(TRUN::parse).unwrap()?;
            start_with_i_frame = MediaInfoGenerator::determine_start_with_i_frame_with_trun(&trun)
          }
          let info = SegmentInfo{
            pts,
            duration,
            url: "",
            bytes: Option::Some(sr.referenced_size),
            offset: Option::Some(offset as u32),
            start_with_i_frame,
          };
          println!("INFO = {:?}", info);
          offset += sr.referenced_size as usize;
          pts += sr.subsegment_duration as u64;
        }
    }
      
    Ok(sidx_box)
  }

  pub fn get_captions(mp4: &[u8]) -> Result<STSD, CustomError> {
    let stsd = STSD::parse(mp4)?;
    let hdlr = HDLR::parse(mp4)?;
    // NOTE (benjamintoofer@gmail.com): Grabbing first mdat. Hopefully that's all we need
    let mdat = get_box("mdat", 0, mp4)?;
    if HandlerType::VIDE.eq(&hdlr.get_handler_type()) {
      let avc1_sample_entry = stsd.read_sample_entry("avc1")
        .map(AVCSampleEntry::parse)?;
      let nal_unit_size = avc1_sample_entry.config.length_size_minus_one + 1;
      // AVCSampleEntry::parse(avc1_sample_entry_data)
    } else if HandlerType::SOUN.eq(&hdlr.get_handler_type()) {
      // Do some audio stuff here
    }
    

    return Ok(stsd);
  }
  // pub fn extract_media_info_from_mp4(mp4: &[u8]) -> MediaInfo {

  // }

  // fn track_info(mp4: &[u8]) -> TrackInfo {

  // }

  // fn segment_info(mp4: &[u8]) -> SegmentInfo {

  // }

  fn determine_start_with_i_frame_with_sap(sidx_ref: &SIDXReference) -> bool {
    // Determine if this reference starts with SAP and has a SAP type of 1 or 2. Type 1 or 2
    // indicates that the sample is an Iframe
    sidx_ref.starts_with_sap && (sidx_ref.sap_type == 1 || sidx_ref.sap_type == 2)
  }

  fn determine_start_with_i_frame_with_trun(trun: &TRUN) -> bool {
    trun.first_sample_flags
      .map(|x|SampleFlag::parse(x))
      .as_mut()
      .map(|f| f.get_sample_depends_on() == 2) // Is an I-Frame
      .unwrap_or(false)
  }

  fn determine_captions_id() {

  }
}