static EXT_TAG_PREFIX: &'static str = "#EXT";

pub enum HLSVersion {
  _4,
  _7,
  // TODO (benjamintoofer@gmail.com): Add version 8
}

pub trait HLSTagWriter {
  fn version(&self, version: HLSVersion) -> Box<dyn HLSTagWriter>;
  fn stream_inf(&self) -> dyn HLSTagWriter;
  fn media(&self) -> dyn HLSTagWriter;
  fn i_frame_inf(&self) -> dyn HLSTagWriter;
  fn independent(&self) -> dyn HLSTagWriter;

  fn end(&self) -> String;
}

pub trait HLSStarter {
  fn start_hls(&self) -> dyn HLSTagWriter;
}

pub struct HLSWriter {

}

impl HLSStarter for HLSWriter {
    fn start_hls(&self) -> Box<dyn HLSTagWriter> {
        todo!()
    }
}

impl HLSTagWriter for HLSWriter {
    fn version(&self, version: HLSVersion) -> Box<dyn HLSTagWriter> {
        todo!()
    }

    fn stream_inf(&self) -> dyn HLSTagWriter {
        todo!()
    }

    fn media(&self) -> dyn HLSTagWriter {
        todo!()
    }

    fn i_frame_inf(&self) -> dyn HLSTagWriter {
        todo!()
    }

    fn independent(&self) -> dyn HLSTagWriter {
        todo!()
    }

    fn end(&self) -> String {
        todo!()
    }
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