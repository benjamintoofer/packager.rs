

pub struct SLConfigDescriptorBuilder {}

impl SLConfigDescriptorBuilder {
  pub fn create_builder() -> SLConfigDescriptorBuilder {
    return SLConfigDescriptorBuilder {}
  }

  pub fn build(&self) -> Vec<u8> {
    vec![
      // SLConfigDescrTag
      0x06,
      // length
      0x80, 0x80, 0x80, 0x01,
      // bit(8) predefined (type 2. I don't know what predefined type 2 means but is required by cmaf spec)
      0x02
    ]
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_sl_config_descriptor_builder() {
    let expected_sl_config_descriptor: [u8; 6] = [
      0x06,
      0x80, 0x80, 0x80, 0x01,
      0x02
    ]; 
    let actual_sl_config_descriptor = SLConfigDescriptorBuilder::create_builder()
      .build();
    assert_eq!(actual_sl_config_descriptor, expected_sl_config_descriptor);
  }
}