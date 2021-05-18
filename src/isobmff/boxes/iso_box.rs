use std::convert::TryInto;
use std::convert::TryFrom;
use std::str;

use crate::error::{construct_error, CustomError};
use crate::error::error_code::{MajorCode, ISOBMFFMinorCode};

pub trait IsoBox {
  fn get_size(&self) -> u32;
  fn get_type(&self) -> &String;
}

pub trait IsoFullBox {
  fn get_version(&self) -> u8;
  fn get_flags(&self) -> u32;
}

pub fn get_init_segment_end(mp4: &[u8]) -> usize {
  let mut lower_bound: usize = 0;
  let mut offset = std::u32::MAX;
    
  while lower_bound < mp4.len() {
    let bound_plus_four = lower_bound + 4;
    let size = mp4[lower_bound..bound_plus_four].as_ref();
    let box_type = str::from_utf8(mp4[bound_plus_four..(bound_plus_four + 4)].as_ref());
    let num = u32::from_be_bytes(size.try_into().expect("slice with incorrect length"));

    if let Ok(box_type_str) = box_type {
      if box_type_str.eq("moof") {
        offset = u32::try_from(lower_bound).expect("cannot convert usize (lower_bound) to u32");
        break;
      }
    }

    let converted_val = usize::try_from(num).expect("cannot convert u32 (num) to usize");
    lower_bound += converted_val;
  }
    
    offset as usize
}

pub fn find_box<'a>(search_box: &str, offset: usize, current_box_data: &'a [u8]) -> Option<&'a [u8]> {
  let mut lower_bound: usize = offset;
    
  while lower_bound < current_box_data.len() {
    let bound_plus_four = lower_bound + 4;
    let size = current_box_data[lower_bound..bound_plus_four].as_ref();
    let box_type = str::from_utf8(current_box_data[bound_plus_four..(bound_plus_four + 4)].as_ref());
    let size = u32::from_be_bytes(size.try_into().expect("slice with incorrect length")) as usize;
    
    if let Ok(box_type_str) = box_type {
      if box_type_str.eq(search_box) {
        return Some(current_box_data[lower_bound..(lower_bound + size)].as_ref())
      }
    }
    lower_bound += size;
  }
  None
}

pub fn get_box<'a>(search_box: &str, offset: usize, current_box_data: &'a [u8]) -> Result<&'a [u8], CustomError> {
  let box_data = find_box(search_box, offset, current_box_data);
    
    if let Some(box_data) = box_data {
      Ok(box_data)
    } else {
      Err(construct_error(
        MajorCode::ISOBMFF,
        Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
        format!("{}: Unable to find box", search_box),
        file!(),
        line!()))
    }
}

// TODO (benjamintoofer@gmail.com): Figure out how to change these tests. We want to remove the use of reading in a file but the 
// init segment is too large to write the bytes here
// #[cfg(test)]
// mod tests {

//   use super::*;
//   use std::fs;

//   #[test]
//   fn test_get_init_segment_end() {
//     let file_path = "./assets/v_frag.mp4";
  
//     let expected_value: usize = 907;
//     let mp4_file = fs::read(file_path);
//     if let Ok(mp4) = mp4_file {
//       assert_eq!(get_init_segment_end(&mp4), expected_value);
//     } else {
//       panic!("mp4 file {:} cannot be opened", file_path);
//     }
//   }

//   #[test]
//   fn should_find_box() {
//     let file_path = "./assets/v_frag.mp4";
  
//     let mp4_file = fs::read(file_path);
//     if let Ok(mp4) = mp4_file {
//       assert!(find_box("sidx", 0, &mp4).is_some());
//     } else {
//       panic!("mp4 file {:} cannot be opened", file_path);
//     }
//   }

//   #[test]
//   fn should_not_find_box() {
//     let file_path = "./assets/v_frag.mp4";
  
//     let mp4_file = fs::read(file_path);
//     if let Ok(mp4) = mp4_file {
//       assert!(find_box("fake", 0, &mp4).is_none());
//     } else {
//       panic!("mp4 file {:} cannot be opened", file_path);
//     }
//   }
// }
