use crate::util;
use super::{DescriptorTags, find_descriptor};
use super::dec_config_descriptor::DecoderConfigDescriptor;

static CLASS: &str = "ESDescriptor";
#[derive(Debug)]
pub struct ESDescriptor {
  id: u16,
  stream_dependence_flag: bool,
  url_flag: bool,
  ocr_stream_flag: bool,
  stream_priority: u8,      // 5 bit
  pub dec_config_descr: DecoderConfigDescriptor,
  depends_on_es_id: Option<u16>,
  url_length: Option<u8>,
  url_string: Option<String> // ??
  // ...
}

impl ESDescriptor {
  pub fn parse(data: &[u8]) -> ESDescriptor{
    let mut start = 2usize;
    // Parse es id
    let id = util::get_u16(data, start)
      .expect(format!("{}.parse.id: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 2;
    // Parse streamDependenceFlag, URL_Flag, OCRstreamFlag, and streamPriority
    let flags = util::get_u8(data, start)
      .expect(format!("{}.parse.flags: cannot get u16 from start = {}",CLASS, start).as_ref());

    let stream_dependence_flag = (flags & 0x80) != 0;
    let url_flag = (flags & 0x40) != 0;
    let ocr_stream_flag = (flags & 0x20) != 0;
    let stream_priority = flags & 0x1F;

    if stream_dependence_flag {
      println!("STREAM DEPENDS")
    }

    if url_flag {
      println!("URL FLAG")
    }

    if ocr_stream_flag {
      println!("OCR STREAM FLAG")
    }
     let dec_config_descr = find_descriptor(DescriptorTags::DECODER_CONFIG_DESC, start + 1, data)
      .and_then(|dec_desc|Some(DecoderConfigDescriptor::parse(dec_desc)))
      .expect("No DecoderConfigDescriptor");
    
    ESDescriptor{
      id,
      stream_dependence_flag,
      url_flag,
      ocr_stream_flag,
      stream_priority,
      dec_config_descr,
      depends_on_es_id: None,
      url_length: None,
      url_string: None,
    }
  }
}

// TODO: https://github.com/video-dev/hls.js/blob/0c5bd8b3e86dea194d0871c49be83f8b4130bfb8/src/remux/mp4-generator.ts#L739
// Generating the ESDescriptor