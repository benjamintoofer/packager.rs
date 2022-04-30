use crate::container::isobmff::configuration_records::avcC::AVCDecoderConfigurationRecordBuilder;
use crate::container::isobmff::nal::{nal_unit::NALUnit, NALType};
use crate::container::isobmff::sample_entry::{
    avc_sample_entry::AVCSampleEntryBuilder, sample_entry::SampleEntryBuilder,
    visual_sample_entry::VisualSampleEntryBuilder,
};
use crate::container::isobmff::BoxBuilder;
use crate::util;
use crate::{
    container::{
        isobmff::{nal::NalRep, HandlerType},
        remux::extractor::TSExtractor,
        transport_stream::pes_packet,
        writer::mp4_writer::{Mp4Writer, SampleInfo},
    },
    error::CustomError,
};

pub struct AVCExtractor {
    sps_nal: Vec<u8>,
    pps_nal: Vec<u8>,
    media_nal: Vec<NalRep>,
    bucket: Vec<u8>,
    signed_comp_offset: bool,
    all_same_timestamps: bool,
    current_pts: u64,
    current_dts: u64,
}

impl TSExtractor for AVCExtractor {
    fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) -> Result<(), CustomError> {
        let mut index: usize = 0;
        let mut nal_start_index = index;
        let pes_payload = pes.payload_data;

        loop {
            if (index + 4) >= pes_payload.len() {
                // If the next 4 bytes is greater than the total payload, add it to the bucket for the next pes payload
                let mut leftover: Vec<u8> =
                    pes_payload[nal_start_index..pes_payload.len()].to_vec();
                self.bucket.append(&mut leftover);
                break;
            }
            let boundary = NALUnit::find_boundary(index, pes_payload);
            if boundary == -1 {
                // If no nal boundary is found, increment the index and continue searching for the next boundary
                index += 1;
                continue;
            }
            let mut nal_unit: Vec<u8> = vec![];
            if !self.bucket.is_empty() {
                nal_unit = self.bucket.clone();
                self.bucket.clear();
            }

            if nal_start_index != index {
                let mut part_payload: Vec<u8> = pes_payload[nal_start_index..index].to_vec();
                nal_unit.append(&mut part_payload);
            }

            if !nal_unit.is_empty() {
                let nal_unit_value = nal_unit[0] & 0x1F;
                let nal_type = NALType::get_type(nal_unit_value)?;

                self.handle_nal_unit(nal_type, &nal_unit);
            }
            index += boundary as usize;
            nal_start_index = index;
        }

        if let Some(pts) = pes.pts {
            // Can assume dts is there because the pes parser will set it if its not there
            let dts = pes.dts.unwrap();
            // Set the flag that the composition offset will be negative. Will set the version in trun to 1
            if dts > pts {
                self.signed_comp_offset = true;
            }

            // Determine if we'll need to set the compoisiton offset in the trun
            if pts != dts {
                self.all_same_timestamps = false;
            }

            self.current_pts = pts;
            self.current_dts = dts;
        }

        Ok(())
    }

    fn is_all_same_timestamps(&self) -> bool {
        self.all_same_timestamps
    }

    fn is_signed_comp_offset(&self) -> bool {
        self.signed_comp_offset
    }

    fn build_sample_entry(&mut self) -> Result<Vec<u8>, CustomError> {
        if self.sps_nal.len() > 0 && self.pps_nal.len() > 0 {
            let sps = self.sps_nal[0..].to_vec();
            let pps = self.pps_nal[0..].to_vec();
            self.sps_nal.clear();
            self.pps_nal.clear();
            return AVCSampleEntryBuilder::create_builder()
                .sample_entry(SampleEntryBuilder::create_builder())
                .visual_sample_entry(VisualSampleEntryBuilder::create_builder().sps(&sps))
                .avc_c(
                    AVCDecoderConfigurationRecordBuilder::create_builder()
                        .sps(&sps)
                        .pps(&pps),
                )
                .build();
        }

        println!(
            "AVCExtractor :: build_sample_entry :: No sps or pps available. Returning empty vector"
        );
        Ok(vec![])
    }

    fn flush_final_media(&mut self) -> Result<(), CustomError> {
        self.media_nal.push(NalRep {
            nal_unit: self.bucket.to_vec(),
            pts: self.current_pts,
            dts: self.current_dts,
        });

        Ok(())
    }

    fn get_timescale(&self) -> u32 {
        90000
    }

    fn get_init_segment(&mut self) -> Result<Vec<u8>, CustomError> {
        // let temp = self.into();
        let sample_entry_data = self.build_sample_entry()?.clone();
        let track_id = 1usize;

        Mp4Writer::create_mp4_writer()
            .timescale(self.get_timescale())
            .handler(HandlerType::VIDE)
            .track_id(track_id)
            .build_init_segment(sample_entry_data)
    }

    fn get_media_segment(&mut self) -> Result<Vec<u8>, CustomError> {
        let media_data = AVCExtractor::convert_nal_units_to_sample_infos(self.media_nal.to_owned());
        let track_id = 1usize;
        let trun_version = if self.is_signed_comp_offset() {1u8} else {0u8};

        let mut writer = Mp4Writer::create_mp4_writer()
            .track_id(track_id)
            .timescale(self.get_timescale())
            .is_all_same_timestamps(self.is_all_same_timestamps())
            .trun_version(trun_version)
            .default_sample_duration(1500)
            .samples(media_data);

        if self.is_all_same_timestamps() {
            writer = writer.default_sample_duration(self.get_default_sample_duration());
        }

        writer.build_media_segment()
    }

    fn get_default_sample_duration(&self) -> u32 {
        // self.get_timescale() / fps
        0
    }
}

impl AVCExtractor {
    pub fn create() -> AVCExtractor {
        AVCExtractor {
            sps_nal: vec![],
            pps_nal: vec![],
            media_nal: vec![],
            bucket: vec![],
            all_same_timestamps: true,
            signed_comp_offset: false,
            current_pts: 0,
            current_dts: 0,
        }
    }

    fn handle_nal_unit(&mut self, nal_type: NALType, nal_unit: &[u8]) {
        match nal_type {
            NALType::SPS => {
                self.sps_nal = nal_unit.to_vec();
            }
            NALType::PPS => self.pps_nal = nal_unit.to_vec(),
            NALType::AUD => {}
            _ => self.media_nal.push(NalRep {
                nal_unit: nal_unit.to_vec(),
                pts: self.current_pts,
                dts: self.current_dts,
            }),
        }
    }

    fn convert_nal_units_to_sample_infos(nal_units: Vec<NalRep>) -> Vec<SampleInfo> {
        let sample_infos: Vec<SampleInfo> = nal_units
            .iter()
            .map(|nu| {
                // Create the sample data
                let nal_size: u32 = nu.nal_unit.len() as u32;
                let nal_size_array = util::transform_u32_to_u8_array(nal_size).to_vec();
                let sample = [
                    vec![
                        nal_size_array[3],
                        nal_size_array[2],
                        nal_size_array[1],
                        nal_size_array[0],
                    ],
                    nu.nal_unit.to_owned(),
                ]
                .concat();
                return SampleInfo {
                    data: sample,
                    dts: nu.dts,
                    pts: nu.pts,
                    sample_flags: None,
                };
            })
            .collect();
        sample_infos
    }
}
