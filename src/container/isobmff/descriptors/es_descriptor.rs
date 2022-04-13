use crate::util;
use crate::container::remux;
use crate::container::isobmff::descriptors::sl_config_descriptor::SLConfigDescriptorBuilder;
use super::{DescriptorTags, dec_config_descriptor::DecoderConfigDescriptorBuilder, find_descriptor};
use super::dec_config_descriptor::DecoderConfigDescriptor;

static CLASS: &str = "ESDescriptor";
// NOTE (benjamintoofer@gmail.com): ESDescriptor IS POORLY DOCUMENTED! Look here
// https://github.com/harjot-oberai/MusicDNA/blob/master/app/src/main/java/org/jaudiotagger/audio/mp4/atom/Mp4EsdsBox.java
// Source file was found here: https://stackoverflow.com/questions/30998150/build-an-esds-box-for-an-mp4-that-firefox-can-play
// 14496-1 7.2.6.5
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

pub struct ESDescriptorBuidler {
  dec_conf_desc_builder: Option<DecoderConfigDescriptorBuilder>
}

impl ESDescriptorBuidler {
  pub fn create_builder() -> ESDescriptorBuidler {
    return ESDescriptorBuidler {
      dec_conf_desc_builder: None,
    }
  }

  pub fn dec_conf_desc(mut self, dec_conf_desc_builder: DecoderConfigDescriptorBuilder) -> ESDescriptorBuidler {
    self.dec_conf_desc_builder = Some(dec_conf_desc_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, crate::error::CustomError> {
    let dec_conf_desc = self.dec_conf_desc_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing dec_conf_desc for ESDescriptorBuidler")))?
      .build()?;
    let sl_config_desc = SLConfigDescriptorBuilder::create_builder().build();
    let size = 
      12 + // header
      8 +
      dec_conf_desc.len() +
      sl_config_desc.len();
    let size_array = util::transform_usize_to_u8_array(size);

    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // esds
          0x65, 0x73, 0x64, 0x73,
          // version/flag
          0x00, 0x00, 0x00, 0x00,
          // ES_DescrTag
          0x03,
          // length
          0x80, 0x80, 0x80, (dec_conf_desc.len() + sl_config_desc.len()) as u8,
          // ES_ID
          0x00, 0x00,
          // streamDependenceFlag(1 bit), URL_Flag(1 bit), OCRstreamFlag(1 bit), streamPriority(5 bit)
          0x00,
        ],
        dec_conf_desc,
        sl_config_desc
      ].concat()
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::container::isobmff::descriptors::aac_audio_specific_config::AACAudioSpecificConfigBuilder;

  #[test]
  fn test_es_descriptor_builder() {
    let expected_es_descriptor: [u8; 51] = [
      // esds
      0x00, 0x00, 0x00, 0x33,
      0x65, 0x73, 0x64, 0x73,
      0x00, 0x00, 0x00, 0x00,
      0x03,
      0x80, 0x80, 0x80, 0x1F,
      0x00, 0x00,
      0x00,
      0x04,
      0x80, 0x80, 0x80, 0x14,
      0x40,
      0x15,
      0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x05,
      0x80, 0x80, 0x80, 0x02,
      0x12, 0x30,
      // SLConfigDescriptor
      0x06,
      0x80, 0x80, 0x80, 0x01,
      0x02
    ]; 
    let actual_es_descriptor = ESDescriptorBuidler::create_builder()
      .dec_conf_desc(
         DecoderConfigDescriptorBuilder::create_builder()
          .aac_audio_specific_config(
            AACAudioSpecificConfigBuilder::create_builder()
              .channel_count(6)
              .sampling_frequency_index(4)
          )
      )
      .build()
      .unwrap();
    assert_eq!(actual_es_descriptor, expected_es_descriptor);
  }
}