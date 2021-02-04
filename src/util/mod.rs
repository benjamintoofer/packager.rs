use std::{array::TryFromSliceError, convert::TryInto};

pub mod logger;

pub fn get_u64(data: &[u8], start: usize, end: usize) -> Result<u64, TryFromSliceError> {
  let slice_data = data[start..end]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u64::from_be_bytes(val)),
    Err(err) => Err(err),
  }
}

pub fn get_u32(data: &[u8], start: usize, end: usize) -> Result<u32, TryFromSliceError> {
  let slice_data = data[start..end]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u32::from_be_bytes(val)),
    Err(err) => Err(err),
  }
}

pub fn get_u16(data: &[u8], start: usize, end: usize) -> Result<u16, TryFromSliceError> {
  let slice_data = data[start..end]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u16::from_be_bytes(val)),
    Err(err) => Err(err),
  }
}

pub fn get_u8(data: &[u8], start: usize, end: usize) -> Result<u8, TryFromSliceError> {
  let slice_data = data[start..end]
    .as_ref()
    .try_into();
  
  match slice_data {
    Ok(val) => Ok(u8::from_be_bytes(val)),
    Err(err) => Err(err),
  }
}