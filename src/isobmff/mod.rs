pub mod boxes;
pub mod sample_entry;
pub mod configuration_records;
pub mod descriptors;
pub mod nal;

#[derive(Debug)]
pub enum HandlerType {
  VIDE,
  SOUN,
  HINT,
  META,
  AUXV
}

impl PartialEq<u32> for HandlerType {
  fn eq(&self, other: &u32) -> bool {
    match self {
        HandlerType::VIDE => 0x76696465 == *other,
        HandlerType::SOUN => 0x736F756E == *other,
        HandlerType::HINT => 0x68696e74 == *other,
        HandlerType::META => 0x6d657461 == *other,
        HandlerType::AUXV => 0x61757876 == *other
    }
  }
}
