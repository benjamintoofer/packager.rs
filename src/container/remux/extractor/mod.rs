use crate::container::remux::extractor::ts::{
    aac_extractor::AACExtractor, avc_extractor::AVCExtractor,
};
use crate::container::transport_stream::{
    elementary_stream_type::ElementaryStreamType, pes_packet::PESPacket,
};
use crate::error::error_code::{MajorCode, RemuxMinorCode};
use crate::error::{construct_error, CustomError};

pub mod mp4;
pub mod ts;

pub trait TSExtractor {
    fn accumulate_pes_payload(&mut self, pes: PESPacket) -> Result<(), CustomError>;
    fn is_all_same_timestamps(self) -> bool;
    fn is_signed_comp_offset(self) -> bool;
    fn build_sample_entry(&mut self) -> Result<Vec<u8>, CustomError>;
    fn flush_final_media(&mut self) -> Result<(), CustomError>;
    // fn listen_for_init_data(&mut self, callback: fn(Vec<u8>));
    // fn listen_for_media_data(&mut self, callback: fn(Vec<SampleInfo>));
    fn get_init_segment(&mut self) -> Result<Vec<u8>, CustomError>;
    fn get_media_segment(&mut self) -> Result<Vec<u8>, CustomError>;
    fn get_timescale(&self) -> u32;
}

pub fn get_ts_extractor(
    es_type: ElementaryStreamType,
) -> Result<Box<dyn TSExtractor>, CustomError> {
    return match es_type {
        ElementaryStreamType::AAC => {
            let extractor = Box::new(AACExtractor::create());
            Ok(extractor)
        }
        ElementaryStreamType::AC3 => {
            todo!("Need to implement AC3 transport stream extractor");
        }
        ElementaryStreamType::E_AC3 => {
            todo!("Need to implement E_AC3 transport stream extractor");
        }
        ElementaryStreamType::H_264 => {
            let extractor = Box::new(AVCExtractor::create());
            Ok(extractor)
        }
        ElementaryStreamType::H_265 => {
            todo!("Need to implement H_265 transport stream extractor");
        }
        ElementaryStreamType::UNKNOWN => {
            return Err(construct_error(
                MajorCode::ISOBMFF,
                Box::new(RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR),
                format!("Uknown elementary stream type. CAn't determine which ts extractor to use"),
                file!(),
                line!(),
            ));
        }
    };
}
