static EXT_TAG_PREFIX: &'static str = "#EXT";
pub struct HLSWriter {

}

impl  HLSWriter {
  pub fn createWrite() -> HLSWriter {
    HLSWriter{}
  }

  pub fn temp(){}
}

fn hls_template() -> String {
  let manifest = String::from(EXT_TAG_PREFIX);

  manifest
}