
struct MOOVBuilder {}

impl MOOVBuilder {
  pub fn create_builder() -> MOOVBuilder {
    return MOOVBuilder{}
  }

  pub fn build(&self) -> &'static [u8] {
    return &[
      // MVHDBuilder
      // TRAKBuilder
      // MVEXBuilder (required for fragmented)
    ]
  }
}