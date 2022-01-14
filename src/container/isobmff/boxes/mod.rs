use std::convert::TryInto;

pub mod iso_box;
pub mod ftyp;
pub mod moov;
pub mod sidx;
pub mod mvhd;
pub mod mdhd;
pub mod hdlr;
pub mod dinf;
pub mod dref;
pub mod stbl;
pub mod stsd;
pub mod stts;
pub mod stsc;
pub mod stsz;
pub mod stco;
pub mod tfhd;
pub mod tkhd;
pub mod tfdt;
pub mod trun;

pub struct SampleFlag {
  flag_data: u32,
  is_leading: Option<u8>,
  sample_depends_on: Option<u8>,
  sample_is_depended_on: Option<u8>,
  sample_has_redundancy: Option<u8>,
  sample_padding_value: Option<u8>,
  sample_is_non_sync_sample: Option<u8>,
  // Skipping sample_degradation_priority for now
}

// ISAU of a SAP of type 1 or 2 as defined in Annex I

impl SampleFlag {
  pub fn parse(flag_data: u32) -> SampleFlag {
    SampleFlag{
      flag_data,
      is_leading: Option::None,
      sample_depends_on: Option::None,
      sample_is_depended_on: Option::None,
      sample_has_redundancy: Option::None,
      sample_padding_value: Option::None,
      sample_is_non_sync_sample: Option::None,
    }
  }

  pub fn get_is_leading(&mut self) -> u8 {
    if self.is_leading.is_none() {
      self.is_leading =  Option::Some(((self.flag_data & 0xC000000) >> 26).try_into().unwrap())
    } 

    return self.is_leading.unwrap();
  }

  pub fn get_sample_depends_on(&mut self) -> u8 {
    if self.sample_depends_on.is_none() {
      self.sample_depends_on =  Option::Some(((self.flag_data & 0x3000000) >> 24).try_into().unwrap())
    } 

    return self.sample_depends_on.unwrap();
  }

  pub fn get_sample_is_depended_on(&mut self) -> u8 {
    if self.sample_is_depended_on.is_none() {
      self.sample_is_depended_on =  Option::Some(((self.flag_data & 0xC00000) >> 22).try_into().unwrap())
    } 

    return self.sample_is_depended_on.unwrap();
  }

  pub fn get_sample_has_redundancy(&mut self) -> u8 {
    if self.sample_has_redundancy.is_none() {
      self.sample_has_redundancy =  Option::Some(((self.flag_data & 0x300000) >> 20).try_into().unwrap())
    } 

    return self.sample_has_redundancy.unwrap();
  }

  pub fn get_sample_padding_value(&mut self) -> u8 {
    if self.sample_padding_value.is_none() {
      self.sample_padding_value =  Option::Some(((self.flag_data & 0xE0000) >> 17).try_into().unwrap())
    } 

    return self.sample_padding_value.unwrap();
  }

  pub fn get_sample_is_non_sync_sample(&mut self) -> u8 {
    if self.sample_is_non_sync_sample.is_none() {
      self.sample_is_non_sync_sample =  Option::Some(((self.flag_data & 0x10000) >> 16).try_into().unwrap())
    } 

    return self.sample_is_non_sync_sample.unwrap();
  }
}

#[cfg(test)]
mod tests {

  use super::SampleFlag;

  #[test]
  fn test_sample_flag_parsing() {
    let mut sample_flag = SampleFlag::parse(0x02000000);
    assert_eq!(sample_flag.get_is_leading(), 0);
    assert_eq!(sample_flag.get_sample_depends_on(), 2);
    assert_eq!(sample_flag.get_sample_is_depended_on(), 0);
    assert_eq!(sample_flag.get_sample_has_redundancy(), 0);
    assert_eq!(sample_flag.get_sample_padding_value(), 0);
    assert_eq!(sample_flag.get_sample_is_non_sync_sample(), 0);
  }

  #[test]
  fn test_something() {
    let mut sample_flag = SampleFlag::parse(0x1010000);
    assert_eq!(sample_flag.get_is_leading(), 0);
    assert_eq!(sample_flag.get_sample_depends_on(), 1);
    assert_eq!(sample_flag.get_sample_is_depended_on(), 0);
    assert_eq!(sample_flag.get_sample_has_redundancy(), 0);
    assert_eq!(sample_flag.get_sample_padding_value(), 0);
    assert_eq!(sample_flag.get_sample_is_non_sync_sample(), 1);
  }
}
