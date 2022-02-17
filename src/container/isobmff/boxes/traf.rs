use crate::container::isobmff::boxes::{tfhd::TFHDBuilder, tfdt::TFDTBuilder, trun::TRUNBuilder};
use crate::container::remux;
use crate::error::CustomError;
use crate::util;

pub struct TRAFBuilder {
  tfhd_builder: Option<TFHDBuilder>,
  tfdt_builder: Option<TFDTBuilder>,
  trun_builder: Option<TRUNBuilder>,
  data_offset: usize,
}

impl TRAFBuilder {
  pub fn create_builder() -> TRAFBuilder {
    TRAFBuilder{
      tfhd_builder: None,
      tfdt_builder: None,
      trun_builder: None,
      data_offset: 0,
    }
  }

  pub fn tfhd(mut self, tfhd_builder: TFHDBuilder) -> TRAFBuilder {
    self.tfhd_builder = Some(tfhd_builder);
    self
  }

  pub fn tfdt(mut self, tfdt_builder: TFDTBuilder) -> TRAFBuilder {
    self.tfdt_builder = Some(tfdt_builder);
    self
  }

  pub fn trun(mut self, trun_builder: TRUNBuilder) -> TRAFBuilder {
    self.trun_builder = Some(trun_builder);
    self
  }

  pub fn set_data_offset(mut self, data_offset: usize) -> TRAFBuilder {
    self.data_offset = data_offset;
    self
  }

  pub fn build(self) -> Result<Vec<u8>, CustomError> {
    let tfhd = self.tfhd_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing tfhd_builder for STBLBuilder")))?
      .build();
    let tfdt = self.tfdt_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing tfdt_builder for STBLBuilder")))?
      .build();
    let data_offset = self.data_offset + tfhd.len() + tfdt.len() + 8;
    let trun = self.trun_builder
      .ok_or_else(||remux::generate_error(String::from("Missing trun_builder for STBLBuilder")))?
      .data_offset(data_offset)
      .build();

    let size = 
      8 + // header
      tfhd.len() +
      tfdt.len() +
      trun.len();
    let size_array = util::transform_usize_to_u8_array(size);

    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // traf
          0x74, 0x72, 0x61, 0x66,
        ],
        tfhd,
        tfdt,
        trun
      ].concat()
    )
    
  }
}