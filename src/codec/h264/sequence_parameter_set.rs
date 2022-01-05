use crate::{error::CustomError, util::bit_reader::BitReader};

struct SequenceParameterSet {

}

impl SequenceParameterSet {
  pub fn parse(data: &[u8]) -> Result<SequenceParameterSet, CustomError> {
    let mut bit_reader = BitReader::create_bit_reader(data);

    let profile_idc = bit_reader.read_bits(8)?;

    println!("{}", profile_idc);

    Ok(SequenceParameterSet{})
  }
}