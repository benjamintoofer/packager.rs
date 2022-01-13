use crate::{error::CustomError, util::bit_reader::BitReader};

#[derive(Eq, PartialEq, Debug)]
pub struct SequenceParameterSet {
  pub profile_idc: u8,                          // 8 bit
  pub constraint_set0_flag: u8,                 // 1 bit
  pub constraint_set1_flag: u8,                 // 1 bit
  pub constraint_set2_flag: u8,                 // 1 bit
  pub constraint_set3_flag: u8,                 // 1 bit
  pub constraint_set4_flag: u8,                 // 1 bit
  pub constraint_set5_flag: u8,                 // 1 bit
  pub level_idc: u8,                            // 8 bit
  pub seq_parameter_set_id: usize,              // variable
  pub log2_max_frame_num_minus4: usize,         // variable
  pub max_num_ref_frames: usize,                // variable
  pub gaps_in_frame_num_value_allowed_flag: u8, // 1 bit
  pub direct_8x8_inference_flag: u8,            // 1 bit
  pub pic_width_in_mbs_minus1: usize,           // variable
  pub pic_height_in_map_units_minus1: usize,    // variable
  pub frame_mbs_only_flag: usize,               // variable

  pub frame_crop_left_offset: usize,
  pub frame_crop_right_offset: usize,
  pub frame_crop_top_offset: usize,
  pub frame_crop_bottom_offset: usize,
}

impl SequenceParameterSet {
  pub fn parse(data: &[u8]) -> Result<SequenceParameterSet, CustomError> {
    let mut bit_reader = BitReader::create_bit_reader(data);
    bit_reader.read_bits(8)?; // skip the nal unit header (forbidden_zero_bit (1), nal_ref_idc(2), nal_unit_type(5))
    let profile_idc = bit_reader.read_bits(8)? as u8;
    let constraint_set0_flag = bit_reader.read_bits(1)? as u8;
    let constraint_set1_flag = bit_reader.read_bits(1)? as u8;
    let constraint_set2_flag = bit_reader.read_bits(1)? as u8;
    let constraint_set3_flag = bit_reader.read_bits(1)? as u8;
    let constraint_set4_flag = bit_reader.read_bits(1)? as u8;
    let constraint_set5_flag = bit_reader.read_bits(1)? as u8;
    bit_reader.read_bits(2)?; // Skip 2 reserved
    let level_idc = bit_reader.read_bits(8)? as u8;
    let seq_parameter_set_id = bit_reader.unsigned_exp_golomb()?;

    if profile_idc == 100 || profile_idc == 110 || profile_idc == 122 ||
       profile_idc == 244 || profile_idc == 44 || profile_idc == 83 ||
       profile_idc == 86 || profile_idc == 118 || profile_idc == 128 ||
       profile_idc == 138 || profile_idc == 139 || profile_idc == 134 ||
       profile_idc == 135
       {
         todo!("Color data for other profiles")
       }
    
    let log2_max_frame_num_minus4 = bit_reader.unsigned_exp_golomb()?;
    let pic_order_cnt_type = bit_reader.unsigned_exp_golomb()?;
    if pic_order_cnt_type == 0 {
      todo!("Need to implement pic_order_cnt_type 0 in the sps")
    } else if pic_order_cnt_type == 1 {
      todo!("Need to implement pic_order_cnt_type 1 in the sps")
    }
    let max_num_ref_frames = bit_reader.unsigned_exp_golomb()?;
    let gaps_in_frame_num_value_allowed_flag = bit_reader.read_bits(1)? as u8;
    let pic_width_in_mbs_minus1 = bit_reader.unsigned_exp_golomb()?;
    let pic_height_in_map_units_minus1 = bit_reader.unsigned_exp_golomb()?;
    let frame_mbs_only_flag = bit_reader.read_bits(1)?;
    if frame_mbs_only_flag == 0 {
      todo!("Need to implement mb_adaptive_frame_field_flag in the sps")
    }
    let direct_8x8_inference_flag = bit_reader.read_bits(1)? as u8;
    let  frame_cropping_flag = bit_reader.read_bits(1)? as u8;
    let  mut frame_crop_left_offset = 0usize;
    let  mut frame_crop_right_offset = 0usize;
    let  mut frame_crop_top_offset = 0usize;
    let  mut frame_crop_bottom_offset = 0usize;
    if frame_cropping_flag == 1 {
      frame_crop_left_offset = bit_reader.unsigned_exp_golomb()?;
      frame_crop_right_offset = bit_reader.unsigned_exp_golomb()?;
      frame_crop_top_offset = bit_reader.unsigned_exp_golomb()?;
      frame_crop_bottom_offset = bit_reader.unsigned_exp_golomb()?;
    }
    Ok(SequenceParameterSet{
      profile_idc,
      constraint_set0_flag,
      constraint_set1_flag,
      constraint_set2_flag,
      constraint_set3_flag,
      constraint_set4_flag,
      constraint_set5_flag,
      level_idc,
      seq_parameter_set_id,
      log2_max_frame_num_minus4,
      max_num_ref_frames,
      gaps_in_frame_num_value_allowed_flag,
      direct_8x8_inference_flag,
      pic_width_in_mbs_minus1,
      pic_height_in_map_units_minus1,
      frame_mbs_only_flag,
      frame_crop_left_offset,
      frame_crop_right_offset,
      frame_crop_top_offset,
      frame_crop_bottom_offset,
    })
  }

  pub fn width(&self) -> usize {
    ((self.pic_width_in_mbs_minus1 + 1) * 16) - self.frame_crop_left_offset * 2 - self.frame_crop_right_offset * 2
  }

  pub fn height(&self) -> usize {
    ((2 - self.frame_mbs_only_flag)* (self.pic_height_in_map_units_minus1 +1) * 16) - (self.frame_crop_top_offset * 2) - (self.frame_crop_bottom_offset * 2)
  }

  pub fn profile_compatability(&self) -> u8 {
    (self.constraint_set0_flag << 7) |
    (self.constraint_set1_flag << 6) |
    (self.constraint_set2_flag << 5) |
    (self.constraint_set3_flag << 4) |
    (self.constraint_set4_flag << 3) |
    (self.constraint_set5_flag << 2)
  }
}

#[cfg(test)]
mod tests {

  fn get_expected_sps() -> SequenceParameterSet {
    return  SequenceParameterSet{
      profile_idc: 66,
      constraint_set0_flag: 1,
      constraint_set1_flag: 1,
      constraint_set2_flag: 0,
      constraint_set3_flag: 0,
      constraint_set4_flag: 0,
      constraint_set5_flag: 0,
      level_idc: 30,
      seq_parameter_set_id: 0,
      log2_max_frame_num_minus4: 0,
      max_num_ref_frames: 3,
      gaps_in_frame_num_value_allowed_flag: 0,
      direct_8x8_inference_flag: 1,
      pic_width_in_mbs_minus1: 29,
      pic_height_in_map_units_minus1: 16,
      frame_mbs_only_flag: 1,
      frame_crop_left_offset: 0,
      frame_crop_right_offset: 0,
      frame_crop_top_offset: 0,
      frame_crop_bottom_offset: 1,
    }
  }
  use super::*;

  #[test]
  fn test_parse_sps() {
    let sps:[u8; 25] = [
      0x67, 0x42, 0xC0, 0x1E, 0xD9, 0x01, 0xE0, 0x8F, 0xEB, 0x01, 0x10, 0x00, 0x00, 0x03, 0x00, 0x10, 0x00, 0x00, 0x03, 0x03, 0xC0, 0xF1, 0x62, 0xE4, 0x80
    ];
    let expected_sps: SequenceParameterSet = get_expected_sps();
    let actual_sps = SequenceParameterSet::parse(&sps).unwrap();
    assert_eq!(actual_sps.width(), 480);
    assert_eq!(actual_sps.height(), 270);
    assert_eq!(actual_sps.profile_compatability(), 192);
    assert_eq!(actual_sps, expected_sps);
  }
}
