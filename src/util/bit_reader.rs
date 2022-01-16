use crate::error::{CustomError, construct_error, error_code::{UtilMinorCode, MajorCode}};


pub struct BitReader<'a> {
  data: &'a [u8],
  data_index: usize,
  word: usize, //64 bit
  bit_counter: usize,
}

impl<'a> BitReader<'a> {

  pub fn create_bit_reader(data: &'a [u8]) -> BitReader {
    let mut bit_reader = BitReader{
      data,
      data_index: 0,
      word: 0,
      bit_counter: 0,
    };
    bit_reader.load_word();
    return bit_reader;
  }

  pub fn read_bits(&mut self, count: usize) -> Result<usize, CustomError> {
    let num_of_bits = self.data.len() * 8;
    if self.data_index + count > num_of_bits {
      return Err(
        construct_error(
          MajorCode::UTIL,
          Box::new(UtilMinorCode::PARSING_BIT_READER_ERROR),
          format!("Could not parse count of {}. Exceeds data count: {}",count, self.data.len()),
          file!(),
          line!()
        )
      )
    }

    if self.bit_counter < count {
      self.load_word()
    }

    let diff = 64 - count;
    let mask = !((1usize << diff) - 1);
    let read_data = (self.word & mask) >> (64 - count);
    
    self.clear_bits(count);
    Ok(read_data)
  }

  fn load_word(&mut self) {

    let mut holder = 0usize;
    let mut index_offset = 0usize;
    let bytes = 8usize;

    let temp_word = loop {
      let index = self.data_index + index_offset;
      if index >= self.data.len() {
        break holder;
      }

      let temp = self.data[index] as usize;
      let offset = bytes - 1 - index_offset;
      holder = holder | (temp << (offset * 8));


      index_offset += 1;

      if index_offset == bytes {
        break holder;
      }
    };

    self.word |= temp_word >> self.bit_counter;
    self.data_index += 8 - (self.bit_counter / 8);
    self.bit_counter = index_offset * 8;
  }

  fn clear_bits(&mut self, count: usize) {
    self.word = self.word << count;
    self.bit_counter -= count;
  }

  pub fn unsigned_exp_golomb(&mut self) -> Result<usize, CustomError> {
    let leading_zero_count = self.leading_zeroes()?;
    if leading_zero_count == 0 {
      return Ok(0)
    }
    let add = self.read_bits(leading_zero_count)?;
    let exp_golomb_value = (1 << leading_zero_count) - 1 + add;
    Ok(exp_golomb_value)
  }

  fn leading_zeroes(&mut self) -> Result<usize, CustomError> {
    let mut leading_zeroes = 0usize;
    let mut b = self.read_bits(1)?;
    return loop {     
      if b == 1 {
        break Ok(leading_zeroes);
      }
      b = self.read_bits(1)?; 
      leading_zeroes += 1;
    };
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_init_bit_reader() {
    let data: [u8; 4] = [
      0x01, 0x02, 0x03, 0x04,
    ];
  
    let mut bit_reader = BitReader::create_bit_reader(&data);
    assert_eq!(bit_reader.word, 0x0102030400000000);
    assert_eq!(bit_reader.bit_counter, 32);

    let data_2: [u8; 8] = [
      0x01, 0x02, 0x03, 0x04,0x05, 0x06, 0x07, 0x08,
    ];
    bit_reader = BitReader::create_bit_reader(&data_2);
    assert_eq!(bit_reader.word, 0x0102030405060708);
    assert_eq!(bit_reader.bit_counter, 64);
  }

  #[test]
  fn test_read_bits() {
    let data: [u8; 8] = [
      0x01, 0x02, 0x03, 0x04,0x05, 0x06, 0x07, 0x08,
    ];
  
    let mut bit_reader = BitReader::create_bit_reader(&data);
    let value = bit_reader.read_bits(24).unwrap();
    assert_eq!(value, 0x010203);
    assert_eq!(bit_reader.bit_counter, 40);
  }

  #[test]
  fn test_read_bits_with_a_load() {
    let data: [u8; 24] = [
      0x01, 0x02, 0x03, 0x04,0x05, 0x06, 0x07, 0x08,
      0x09, 0x0a, 0x0b, 0x0c,0x0d, 0x0e, 0x0f, 0x10,
      0x20, 0x30, 0x40, 0x50,0x60, 0x70, 0x80, 0x90,
    ];

    let mut bit_reader = BitReader::create_bit_reader(&data);
    let mut value = bit_reader.read_bits(24).unwrap();
    assert_eq!(value, 0x010203);
    assert_eq!(bit_reader.bit_counter, 40);

    value = bit_reader.read_bits(32).unwrap();
    assert_eq!(value, 0x04050607);
    assert_eq!(bit_reader.bit_counter, 8);

    value = bit_reader.read_bits(16).unwrap();
    assert_eq!(value, 0x0809);
    assert_eq!(bit_reader.bit_counter, 48);

    value = bit_reader.read_bits(45).unwrap();
    assert_eq!(value, 0x1416181A1C1);
    assert_eq!(bit_reader.bit_counter, 3);

    value = bit_reader.read_bits(11).unwrap();
    assert_eq!(value, 0x710);
    assert_eq!(bit_reader.bit_counter, 53);
  }

  #[test]
  fn test_unsigned_exp_golomb() {
    let data: [u8; 4] = [
      0b00100110u8, 
      0b11011010u8, 
      0b11000100u8,
      0b10010100u8
    ];
    
    let mut bit_reader = BitReader::create_bit_reader(&data);
    let mut value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 3);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 0);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 0);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 2);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 2);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 1);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 0);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 0);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 8);
    value = bit_reader.unsigned_exp_golomb().unwrap();
    assert_eq!(value, 4);
  }
}