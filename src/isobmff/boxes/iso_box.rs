use std::convert::TryInto;
use std::convert::TryFrom;
use std::str;

pub trait IsoBox {
  fn get_size(&self) -> u32;
  fn get_type(&self) -> &String;
}

pub trait IsoFullBox {
  fn get_version(&self) -> u8;
  fn get_flags(&self) -> u32;
}

pub fn get_init_segment_end(mp4: &Vec<u8>) -> usize {
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

// TODO (benjamintoofer@gmail.com): Test for get_init_segment_end
