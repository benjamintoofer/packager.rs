
use crate::util;

static CLASS: &str = "AVCDecoderConfigurationRecord";

#[derive(Debug)]
pub struct AVCDecoderConfigurationRecord {
  configuration_version: u8,
  avc_profile_indication: u8,
  profile_compatability: u8,
  avc_level_indication: u8,
  pub length_size_minus_one: u8,            // 2 bits
  num_of_sequence_parameter_sets: u8,   // 5 bits
  num_of_picture_parameter_sets: u8,    // 8 bits
}

impl  AVCDecoderConfigurationRecord {
  pub fn parse(data: &[u8]) -> AVCDecoderConfigurationRecord {
    let mut start = 8usize;
    // Parse configuration version
    let configuration_version = util::get_u8(data, start)
      .expect(format!("{}.parse.configuration_version: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    // Parse configuration version
    let avc_profile_indication = util::get_u8(data, start)
      .expect(format!("{}.parse.avc_profile_indication: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    // Parse profile compatability
    let profile_compatability = util::get_u8(data, start)
      .expect(format!("{}.parse.profile_compatability: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    // Parse AVC level indication
    let avc_level_indication = util::get_u8(data, start)
      .expect(format!("{}.parse.avc_level_indication: cannot get u8 from start = {}",CLASS, start).as_ref());

    start = start + 1;
    // Parse length size minus one
    let length_size_minus_one = util::get_u8(data, start)
      .expect(format!("{}.parse.length_size_minus_one: cannot get u8 from start = {}",CLASS, start).as_ref());
    let length_size_minus_one = length_size_minus_one & 0x2;

    start = start + 1;
    // Parse num of sequence parameter sets
    let num_of_sequence_parameter_sets = util::get_u8(data, start)
      .expect(format!("{}.parse.num_of_sequence_parameter_sets: cannot get u8 from start = {}",CLASS, start).as_ref());
    let num_of_sequence_parameter_sets = num_of_sequence_parameter_sets & 0x5;

    // Parse num of pictures parameter sets
    // todo

    AVCDecoderConfigurationRecord {
      configuration_version,
      avc_profile_indication,
      profile_compatability,
      avc_level_indication,
      length_size_minus_one,
      num_of_sequence_parameter_sets,
      num_of_picture_parameter_sets: 0u8
    }
  }
}