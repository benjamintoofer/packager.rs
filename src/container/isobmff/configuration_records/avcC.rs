
use crate::{codec::h264::sequence_parameter_set::SequenceParameterSet, error::CustomError, util};

static CLASS: &str = "AVCDecoderConfigurationRecord";

/// AVCDecoderConfigurationRecord: 14496-15; 5.2.4.1
#[derive(Debug)]
pub struct AVCDecoderConfigurationRecord {
  pub configuration_version: u8,
  pub avc_profile_indication: u8,
  pub profile_compatability: u8,
  pub avc_level_indication: u8,
  pub length_size_minus_one: u8,        // 2 bits
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

pub struct AVCDecoderConfigurationRecordBuilder {
  sps_data: Vec<u8>,
  pps_data: Vec<u8>,
}

impl AVCDecoderConfigurationRecordBuilder {
  pub fn create_builder() -> AVCDecoderConfigurationRecordBuilder {
    return AVCDecoderConfigurationRecordBuilder{
      pps_data: vec![],
      sps_data: vec![],
    }
  }

  pub fn pps(mut self, pps_data: &[u8]) -> AVCDecoderConfigurationRecordBuilder {
    self.pps_data = pps_data.to_vec();
    self
  }

  pub fn sps(mut self, sps_data: &[u8]) -> AVCDecoderConfigurationRecordBuilder {
    self.sps_data = sps_data.to_vec();
    self
  }
  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    // sps data
    let sps = SequenceParameterSet::parse(&self.sps_data)?;
    let sps_length = self.sps_data.len();
    let sps_length_array = util::transform_usize_to_u8_array(sps_length);

    // pps data
    let pps_length = self.pps_data.len();
    let pps_length_array = util::transform_usize_to_u8_array(pps_length);
    // calculate size
    let size = 
      8 + // header
      8 +
      sps_length + 
      3 + 
      pps_length;
    let size_array = util::transform_usize_to_u8_array(size);
    let avcC: Vec<u8> = [
      vec![
        // size
        size_array[3], size_array[2], size_array[1], size_array[0],
        // avcC
        0x61, 0x76, 0x63, 0x43,
        // configurationVersion
        0x01,
        // AVCProfileIndication
        sps.profile_idc,
        // profile_compatibility
        sps.profile_compatability(),
        // AVCLevelIndication
        sps.level_idc,
        // reserved = ‘111111’b + lengthSizeMinusOne = `11`b = 3
        0xFF,
        // reserved = ‘111’b + numOfSequenceParameterSets = `1`b = 1
        0xE1,
        // sequenceParameterSetLength
        sps_length_array[1], sps_length_array[0],
      ],
      self.sps_data,
      vec![
        // numOfPictureParameterSets
        0x01,
        // pictureParameterSetLength
        pps_length_array[1], pps_length_array[0],
      ],
      self.pps_data,
    ].concat();
    
    Ok(avcC)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_avcC() {
    let expected_avcC = vec![
      0x00, 0x00, 0x00, 0x30, 0x61, 0x76, 0x63, 0x43, 0x01, 0x42, 0xC0, 0x1E, 0xFF, 0xE1, 0x00, 0x19, 0x67, 0x42, 0xC0, 0x1E, 0xD9, 0x01, 0xE0, 0x8F, 0xEB, 0x01, 0x10, 0x00, 0x00, 0x03, 0x00, 0x10, 0x00, 0x00, 0x03, 0x03, 0xC0, 0xF1, 0x62, 0xE4, 0x80, 0x01, 0x00, 0x04, 0x68, 0xCB, 0x8C, 0xB2
    ];
    let sps: [u8; 25] = [
      0x67, 0x42, 0xC0, 0x1E, 0xD9, 0x01, 0xE0, 0x8F, 0xEB, 0x01, 0x10, 0x00, 0x00, 0x03, 0x00, 0x10, 0x00, 0x00, 0x03, 0x03, 0xC0, 0xF1, 0x62, 0xE4, 0x80
    ];
    let pps: [u8; 4] = [
      0x68, 0xcb, 0x8c, 0xb2
    ];

    let avcC = AVCDecoderConfigurationRecordBuilder::create_builder()
      .sps(&sps)
      .pps(&pps)
      .build()
      .unwrap();
    
    assert_eq!(avcC, expected_avcC);
  }
}
