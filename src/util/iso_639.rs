use std::str;

use crate::util::bit_reader::BitReader;
use crate::error::{CustomError, construct_error, error_code::MajorCode, error_code::UtilMinorCode};

pub struct ISO639 {}
 
impl ISO639 {
  pub fn adjust_iso_639_2_to_string(data: &[u8; 2]) -> Result<String, CustomError> {
    let mut bit_reader = BitReader::create_bit_reader(data);
    bit_reader.read_bits(1)?;
    let mut characters:[u8; 3] = [0x00, 0x00, 0x00];
    characters[0] = bit_reader.read_bits(5)? as u8 + 0x60;
    characters[1] = bit_reader.read_bits(5)? as u8 + 0x60;
    characters[2] = bit_reader.read_bits(5)? as u8 + 0x60;

    match str::from_utf8(&characters) {
        Ok(v) => return Ok(String::from(v)),
        Err(e) => return Err(construct_error(
          MajorCode::UTIL,
          Box::new(UtilMinorCode::INVALID_ISO_639_2_CODE_ERROR),
          format!("{}", e),
          file!(),
          line!())
        ),
    };
  }

  pub fn adjust_string_to_iso_639_2(language_code: &str) -> Result<[u8; 2], CustomError> {
    if language_code.len() != 3 {
      return Err(
        construct_error(
          MajorCode::UTIL,
          Box::new(UtilMinorCode::INVALID_ISO_639_2_CODE_ERROR),
          format!("language code is of the incorrect length {}", language_code.len()),
          file!(),
          line!())
      )
    }

    let temp = language_code.as_bytes();
    let added_together: u16 = (temp[0] as u16 - 0x60) << 10 | (temp[1] as u16 - 0x60) << 5 | temp[2] as u16 - 0x60;
    Ok([((added_together & 0xFF00) >> 8) as u8, (added_together & 0xFF) as u8])
  }

  /// Convert ISO-639-2/T language code to a name
  pub fn map_iso_639_2_to_name(language_code: &String) -> String {
    match language_code.as_str() {
      "eng" => {"English".to_string()}
      _ => {"Unknown".to_string()}
    }
  }

  /// Convert ISO-639-2/T language code to a ISO-639-1/T language code
  pub fn map_iso_639_2_to_639_1(language_code: &String) -> String {
    match language_code.as_str() {
      "eng" => {"en".to_string()}
      _ => {"un".to_string()}
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_map_iso_639_2_to_name() {
    assert_eq!(ISO639::map_iso_639_2_to_name(&String::from("eng")), String::from("English"));
    assert_eq!(ISO639::map_iso_639_2_to_name(&String::from("und")), String::from("Unknown"));
    assert_eq!(ISO639::map_iso_639_2_to_name(&String::from("random")), String::from("Unknown"));
  }

  #[test]
  fn test_map_iso_639_2_to_639_1() {
    assert_eq!(ISO639::map_iso_639_2_to_639_1(&String::from("eng")), String::from("en"));
    assert_eq!(ISO639::map_iso_639_2_to_639_1(&String::from("und")), String::from("un"));
  }

  #[test]
  fn test_adjust_iso_639_2_to_string() {
    assert_eq!(ISO639::adjust_iso_639_2_to_string(&[0x55, 0xC4]).unwrap(), String::from("und"));
  }

  #[test]
  fn test_adjust_string_to_iso_639_2() {
    assert_eq!(ISO639::adjust_string_to_iso_639_2("und").unwrap(), [0x55, 0xC4]);
  }
}