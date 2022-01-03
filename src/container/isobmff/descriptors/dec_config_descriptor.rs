use crate::util;

use super::find_descriptor;
use super::DescriptorTags;
use super::aac_audio_specific_config::AACAudioSpecificConfig;

// 14496-1; 7.2.6.6
static CLASS: &str = "DecoderConfigDescriptor";
#[derive(Debug)]
pub struct DecoderConfigDescriptor {
  pub object_type_indication: u8,
  stream_type: u8,            // 6 bit
  upstream: bool,             // 1 bit
  buffer_size_db: u32,         // 24 bit
  max_bitrate: u32,
  avg_bitrate: u32,
  pub audio_sepcific_info: AACAudioSpecificConfig
}

impl  DecoderConfigDescriptor {
  pub fn parse(data: &[u8]) -> DecoderConfigDescriptor {
    let mut start = 2usize;
    // Parse object_type_indication
    let object_type_indication = util::get_u8(data, start)
      .expect(format!("{}.parse.object_type_indication: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    let temp = util::get_u8(data, start)
      .expect(format!("{}.parse.temp: cannot get u8 from start = {}",CLASS, start).as_ref());
    let stream_type = (temp & 0xFC) >> 2;
    let upstream = (temp & 0x2) != 0;

    let mut buffer_size_db: u32 = 0;
    for i in 0..3 {
      start = start +1;
      let buff = util::get_u8(data, start)
        .expect(format!("{}.parse.buff: cannot get u8 from start = {}",CLASS, start).as_ref());
        buffer_size_db =  buffer_size_db | (u32::from(buff) << (8 * (2 - i)));
    }

    start = start + 1;
    let max_bitrate = util::get_u32(data, start)
      .expect(format!("{}.parse.temp: cannot get u32 from start = {}",CLASS, start).as_ref());

    start = start + 4;
    let avg_bitrate = util::get_u32(data, start)
      .expect(format!("{}.parse.avg_bitrate: cannot get u32 from start = {}",CLASS, start).as_ref());

    let audio_sepcific_info = find_descriptor(DescriptorTags::DEC_SPECIFIC_INFO, start + 4, data)
      .and_then(|dec_info|Some(AACAudioSpecificConfig::parse(dec_info)))
      .expect("No DecoderConfigDescriptor")
      .unwrap();
    DecoderConfigDescriptor {
      object_type_indication,
      stream_type,
      upstream,
      buffer_size_db,
      max_bitrate,
      avg_bitrate,
      audio_sepcific_info
    }
  }
}