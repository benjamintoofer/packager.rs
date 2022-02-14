pub mod logger;
pub mod bit_reader;
pub mod iso_639;

use std::{convert::TryInto};
use crate::error::{error_code:: {MajorCode, UtilMinorCode}, construct_error, CustomError};

/**
 * Unsigned operations
 */

pub fn get_u64(data: &[u8], start: usize) -> Result<u64, CustomError> {
  if data.len() < 8 {
    return Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        "Invalid data length to parse u64".to_string(), 
        file!(), 
        line!())
    );
  }
  let slice_data = data[start..(start + 8)]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u64::from_be_bytes(val)),
    Err(err) => Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        err.to_string(), 
        file!(), 
        line!())
    )
  }
}

pub fn get_u32(data: &[u8], start: usize) -> Result<u32, CustomError> {
  if data.len() < 4 {
    return Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        "Invalid data length to parse u32".to_string(), 
        file!(), 
        line!())
    );
  }
  let slice_data = data[start..(start + 4)]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u32::from_be_bytes(val)),
    Err(err) => Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        err.to_string(), 
        file!(), 
        line!())
    )
  }
}

pub fn get_u16(data: &[u8], start: usize) -> Result<u16, CustomError> {
  if data.len() < 2 {
    return Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        "Invalid data length to parse u16".to_string(), 
        file!(), 
        line!())
    );
  }
  let slice_data = data[start..(start + 2)]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u16::from_be_bytes(val)),
    Err(err) => Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        err.to_string(), 
        file!(), 
        line!())
    )
  }
}

pub fn get_u8(data: &[u8], start: usize) -> Result<u8, CustomError> {
  if data.len() == 0 {
    return Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        "No data available to parse u8".to_string(), 
        file!(), 
        line!())
    );
  }
  let slice_data = data[start];
  Ok(slice_data)
}

/**
 * Signed operations
 */
pub fn get_i32(data: &[u8], start: usize)-> Result<i32, CustomError> {
   if data.len() < 4 {
    return Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        "Invalid data length to parse i32".to_string(), 
        file!(), 
        line!())
    );
  }
  let slice_data = data[start..(start + 4)]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(i32::from_be_bytes(val)),
    Err(err) => Err(
      construct_error(
        MajorCode::UTIL, 
        Box::new(UtilMinorCode::PARSING_UNSIGNED_ERROR),
        err.to_string(), 
        file!(), 
        line!())
    )
  }
}

/**
 * Transform unsigned to u8 array
 */
pub fn transform_usize_to_u8_array(x:usize) -> [u8;8] {
  let b1 : u8 = ((x >> 56) & 0xff) as u8;
  let b2 : u8 = ((x >> 48) & 0xff) as u8;
  let b3 : u8 = ((x >> 40) & 0xff) as u8;
  let b4 : u8 = ((x >> 32) & 0xff) as u8;
  let b5 : u8 = ((x >> 24) & 0xff) as u8;
  let b6 : u8 = ((x >> 16) & 0xff) as u8;
  let b7 : u8 = ((x >> 8) & 0xff) as u8;
  let b8 : u8 = (x & 0xff) as u8;
  [b8, b7, b6, b5, b4, b3, b2, b1]
}

pub fn transform_i32_to_u8_array(x:i32) -> [u8;4] {
  let b1 : u8 = ((x >> 24) & 0xff) as u8;
  let b2 : u8 = ((x >> 16) & 0xff) as u8;
  let b3 : u8 = ((x >> 8) & 0xff) as u8;
  let b4 : u8 = (x & 0xff) as u8;
  [b4, b3, b2, b1]
}

#[cfg(test)]
mod tests {

  use std::assert_eq;

    use super::*;

  /* u64 tests */
  #[test]
  fn test_get_u64_ok() {
    let val: [u8; 9] = [1,0,0,0,0,0,0,0,0];
    match get_u64(&val, 0) {
        Ok(val) => {assert_eq!(val, 72057594037927936)}
        Err(_) => {panic!("Error")}
    }
  }

  #[test]
  fn test_get_u64_error() {
    let val: [u8; 7] = [1,0,0,0,0,0,0];
    assert_eq!(get_u64(&val, 0).is_err(), true)
  }

  /* u32 tests */
  #[test]
  fn test_get_u32_ok() {
    let val: [u8; 5] = [1,0,0,0,0];
    match get_u32(&val, 0) {
        Ok(val) => {assert_eq!(val, 16777216)}
        Err(_) => {panic!("Error")}
    }
  }

  #[test]
  fn test_get_u32_error() {
    let val: [u8; 3] = [1,0,0];
    assert_eq!(get_u32(&val, 0).is_err(), true)
  }

  /* u16 tests */
  #[test]
  fn test_get_u16_ok() {
    let val: [u8; 3] = [1,0,0];
    match get_u16(&val, 0) {
        Ok(val) => {assert_eq!(val, 256)}
        Err(_) => {panic!("Error")}
    }
  }

  #[test]
  fn test_get_u16_error() {
    let val: [u8; 1] = [1];
    assert_eq!(get_u16(&val, 0).is_err(), true)
  }

  /* u8 tests */
  #[test]
  fn test_get_u8_ok() {
    let val: [u8; 2] = [1,0];
    match get_u8(&val, 0) {
        Ok(val) => {assert_eq!(val, 1)}
        Err(_) => {panic!("Error")}
    }
  }

  #[test]
  fn test_get_u8_error() {
    let val: [u8; 0] = [];
    assert_eq!(get_u8(&val, 0).is_err(), true)
  }

  /* i32 tests */
  #[test]
  fn test_get_i32_ok() {
    let val: [u8; 8] = [0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff];
    match get_i32(&val, 0) {
        Ok(val) => {assert_eq!(val, -1)}
        Err(_) => {panic!("Error")}
    }
  }

  #[test]
  fn test_get_i32_error() {
    let val: [u8; 3] = [1,0,0];
    assert_eq!(get_i32(&val, 0).is_err(), true)
  }
}