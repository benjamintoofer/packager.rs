use crate::util;
use crate::error::CustomError;
use crate::container::remux;
use crate::container::isobmff::boxes::mvhd::MVHDBuilder;
use crate::container::isobmff::boxes::trak::TRAKBuilder;
use crate::container::isobmff::boxes::mvex::MVEXBuilder;

// MovieBox 14496-12; 8.2.1

pub struct MOOVBuilder {
  mvhd_builder: Option<MVHDBuilder>,
  trak_builder: Option<TRAKBuilder>,
  mvex_builder: Option<MVEXBuilder>,
}

impl MOOVBuilder {
  pub fn create_builder() -> MOOVBuilder {
    return MOOVBuilder{
      mvhd_builder: None,
      trak_builder: None,
      mvex_builder: None,
    }
  }

  pub fn mvhd(mut self, mvhd_builder: MVHDBuilder) -> MOOVBuilder {
    self.mvhd_builder = Some(mvhd_builder);
    self
  }

  pub fn trak(mut self, trak_builder: TRAKBuilder) -> MOOVBuilder {
    self.trak_builder = Some(trak_builder);
    self
  }

  pub fn mvex(mut self, mvex_builder: MVEXBuilder) -> MOOVBuilder {
    self.mvex_builder = Some(mvex_builder);
    self
  }

  pub fn build(&self) -> Result<Vec<u8>, CustomError> {
    let mvhd = self.mvhd_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing mvhd_builder for MOOVBuilder")))?
      .build();
    let trak = self.trak_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing trak_builder for MOOVBuilder")))?
      .build()?;
    let mvex = self.mvex_builder.as_ref()
      .ok_or_else(||remux::generate_error(String::from("Missing mvex_builder for MOOVBuilder")))?
      .build()?;
    
    let size = 
      8 + // header
      mvhd.len() + 
      trak.len() + 
      mvex.len();
    let size_array = util::transform_usize_to_u8_array(size);
    
    Ok(
      [
        vec![
          // size
          size_array[3], size_array[2], size_array[1], size_array[0],
          // moov
          0x6D, 0x6F, 0x6F, 0x76
        ],
        mvhd,
        trak,
        mvex
      ].concat()
    )
  }
}