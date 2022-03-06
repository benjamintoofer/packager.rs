use crate::{container::transport_stream::{elementary_stream_type::ElementaryStreamType, pes_packet:: PESPacket}, error::CustomError};

pub mod ts;
pub mod mp4;

pub trait TSExtractor {
  fn accumulate_pes_payload(&mut self, pes: PESPacket) -> Result<(), CustomError>;
  fn is_all_same_timestamps(self) -> bool;
  fn is_signed_comp_offset(self) -> bool;
  fn build_sample_entry(self) -> Vec<u8>;
  fn flush_final_media(&mut self) -> Result<(), CustomError>;
}

pub fn get_ts_extractor(es_type: ElementaryStreamType) -> Result<Box<dyn TSExtractor>, CustomError> {
  return match es_type {
      ElementaryStreamType::AAC => {}
      ElementaryStreamType::AC3 => {
        todo!("Need to implement AC3 transport stream extractor");
      }
      ElementaryStreamType::E_AC3 => {
        todo!("Need to implement E_AC3 transport stream extractor");
      }
      ElementaryStreamType::H_264 => {}
      ElementaryStreamType::H_265 => {
        todo!("Need to implement H_265 transport stream extractor");
      }
      ElementaryStreamType::UNKNOWN => {
        return Err(

        );
      }
  }
}