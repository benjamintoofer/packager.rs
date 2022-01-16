use crate::util;

// TrackExtendsBox 14496-12; 8.8.3

pub struct TREXBuilder {
  track_id: usize,
  default_sample_duration: usize,
  default_sample_size: usize,
  default_sample_flags: usize,
}

impl TREXBuilder {
  pub fn create_builder() -> TREXBuilder {
    TREXBuilder{
      track_id: 1,
      default_sample_duration: 0,
      default_sample_size: 0,
      default_sample_flags: 0,
    }
  }

  pub fn track_id(mut self, track_id: usize) -> TREXBuilder {
    self.track_id = track_id;
    self
  }

  pub fn default_sample_duration(mut self, default_sample_duration: usize) -> TREXBuilder {
    self.default_sample_duration = default_sample_duration;
    self
  }

  pub fn default_sample_size(mut self, default_sample_size: usize) -> TREXBuilder {
    self.default_sample_size = default_sample_size;
    self
  }

  pub fn default_sample_flags(mut self, default_sample_flags: usize) -> TREXBuilder {
    self.default_sample_flags = default_sample_flags;
    self
  }

  pub fn build(&self) -> Vec<u8> {
    let tid_array = util::transform_usize_to_u8_array(self.track_id);
    let size_array = util::transform_usize_to_u8_array(self.default_sample_size);
    let duration_array = util::transform_usize_to_u8_array(self.default_sample_duration);
    let flags_array = util::transform_usize_to_u8_array(self.default_sample_flags);
    vec![
      // Size
      0x00, 0x00, 0x00, 0x20,
      // trex
      0x74, 0x72, 0x65, 0x78,
      // version
      0x00,
      // flag
      0x00, 0x00, 0x00,
      // track id
      tid_array[3], tid_array[2], tid_array[1], tid_array[0],
      // default_sample_description_index
      0x00, 0x00, 0x00, 0x01,
      // default_sample_duration
      duration_array[3], duration_array[2], duration_array[1], duration_array[0],
      // default_sample_size
      size_array[3], size_array[2], size_array[1], size_array[0],
      // default_sample_flags
      flags_array[3], flags_array[2], flags_array[1], flags_array[0],
    ]
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_trex() {
    let expected_trex: [u8; 32] = [
      0x00, 0x00, 0x00, 0x20,
      0x74, 0x72, 0x65, 0x78,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x02,
      0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    ];
    let trex = TREXBuilder::create_builder()
      .track_id(2)
      .build();
    assert_eq!(trex, expected_trex);
  }
}