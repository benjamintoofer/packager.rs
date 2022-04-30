// "Independent and Disposable Samples Box"
use std::str;

use crate::error::{
    construct_error,
    error_code::{ISOBMFFMinorCode, MajorCode},
    CustomError,
};
use crate::util;
use crate::{
    container::writer::mp4_writer::SampleInfo,
    iso_box::{find_box, IsoBox, IsoFullBox},
};

static CLASS: &str = "TRUN";

#[derive(Debug, Eq)]
struct Sample {
    // All optional fields
    sample_duration: Option<u32>,
    sample_size: Option<u32>,
    sample_flags: Option<u32>,
    sample_composition_time_offset: Option<i32>,
}

impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        self.sample_duration == other.sample_duration
            && self.sample_size == other.sample_size
            && self.sample_flags == other.sample_flags
            && self.sample_composition_time_offset == other.sample_composition_time_offset
    }
}

#[derive(Debug, Eq)]
pub struct TRUN {
    size: u32,
    box_type: String,
    version: u8,
    flags: u32, // u24
    pub sample_count: u32,
    // Optional fields
    data_offset: Option<i32>,
    pub first_sample_flags: Option<u32>,
    samples: Vec<Sample>,
}

impl IsoBox for TRUN {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_type(&self) -> &String {
        &self.box_type
    }
}

impl IsoFullBox for TRUN {
    fn get_version(&self) -> u8 {
        self.version
    }

    fn get_flags(&self) -> u32 {
        self.flags
    }
}

impl PartialEq for TRUN {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.flags == other.flags
            && self.sample_count == other.sample_count
    }
}

impl TRUN {
    pub fn parse(moof: &[u8]) -> Result<TRUN, CustomError> {
        let trun_option = find_box("traf", 8, moof).and_then(|traf| find_box("trun", 8, traf));

        if let Some(trun_data) = trun_option {
            Ok(TRUN::parse_trun(trun_data)?)
        } else {
            Err(construct_error(
                MajorCode::ISOBMFF,
                Box::new(ISOBMFFMinorCode::UNABLE_TO_FIND_BOX_ERROR),
                format!("{}: Unable to find box", CLASS),
                file!(),
                line!(),
            ))
        }
    }

    pub fn parse_trun(trun_data: &[u8]) -> Result<TRUN, CustomError> {
        let mut start = 0usize;

        // Parse size
        let size = util::get_u32(trun_data, start)?;

        start += 4;
        let end = start + 4;
        let box_type = str::from_utf8(trun_data[start..end].as_ref());

        let box_type = match box_type {
            Ok(box_type_str) => String::from(box_type_str),
            Err(err) => panic!("{}", err),
        };

        // Parse flags
        start += 4;
        let flags = util::get_u32(trun_data, start)? & 0xFFFFFF;

        start += 4;
        let sample_count = util::get_u32(trun_data, start)?;
        start += 4;

        // data-offset-present
        let mut data_offset: Option<i32> = Option::None;
        if (flags & 0x000001) != 0 {
            data_offset = Option::Some(util::get_i32(trun_data, start)?);
            start += 4;
        }

        // first-sample-flags-present
        let mut first_sample_flags: Option<u32> = Option::None;
        if (flags & 0x000004) != 0 {
            first_sample_flags = Option::Some(util::get_u32(trun_data, start)?);
            start += 4;
        }

        let mut samples: Vec<Sample> = vec![];
        for _ in 0..sample_count {
            // sample-duration-present
            let mut sample_duration: Option<u32> = Option::None;
            if (flags & 0x000100) != 0 {
                sample_duration = Option::Some(util::get_u32(trun_data, start)?);
                start += 4;
            }

            // sample-size-present
            let mut sample_size: Option<u32> = Option::None;
            if (flags & 0x000200) != 0 {
                sample_size = Option::Some(util::get_u32(trun_data, start)?);
                start += 4;
            }

            // sample-flags-present
            let mut sample_flags: Option<u32> = Option::None;
            if (flags & 0x000400) != 0 {
                sample_flags = Option::Some(util::get_u32(trun_data, start)?);
                start += 4;
            }

            // sample-composition-time-offsets-present
            let mut sample_composition_time_offset: Option<i32> = Option::None;
            if (flags & 0x000800) != 0 {
                sample_composition_time_offset = Option::Some(util::get_i32(trun_data, start)?);
                start += 4;
            }

            samples.push(Sample {
                sample_duration,
                sample_size,
                sample_flags,
                sample_composition_time_offset,
            })
        }

        Ok(TRUN {
            size,
            box_type,
            version: 0,
            flags,
            sample_count,
            data_offset,
            first_sample_flags,
            samples,
        })
    }
}

pub struct TRUNBuilder {
    version: usize,
    flags: usize,
    data_offset: usize,
    first_sample_flags: Option<usize>,
    samples: Vec<SampleInfo>,
    sample_composition_time_offsets_present: bool,
}

impl TRUNBuilder {
    pub fn create_builder() -> TRUNBuilder {
        TRUNBuilder {
            version: 0,
            flags: 0,
            data_offset: 0,
            first_sample_flags: None,
            samples: vec![],
            sample_composition_time_offsets_present: false,
        }
    }

    pub fn samples(mut self, samples: Vec<SampleInfo>) -> TRUNBuilder {
        self.samples = samples;
        self
    }

    pub fn version(mut self, version: usize) -> TRUNBuilder {
        self.version = version;
        self
    }

    pub fn flags(mut self, flags: usize) -> TRUNBuilder {
        self.flags = flags;
        self
    }

    pub fn data_offset(mut self, data_offset: usize) -> TRUNBuilder {
        self.data_offset = data_offset;
        self
    }

    pub fn first_sample_flags(mut self, first_sample_flags: usize) -> TRUNBuilder {
        self.first_sample_flags = Some(first_sample_flags);
        self
    }

    pub fn sample_composition_time_offsets_present(mut self, present: bool) -> TRUNBuilder {
        self.sample_composition_time_offsets_present = present;
        self
    }

    /// Generate the flag and values if they are present
    fn generate_flag(&self) -> (u32, Vec<u8>) {
        // Always start with data-offset-present set. Required for CMAF.
        let mut flag: u32 = 0x00000001;
        let mut data: Vec<u8> = vec![];

        // first-sample-flags-present
        if let Some(first_sample_flags) = self.first_sample_flags {
            flag += 0x000004;
            let array_val = util::transform_usize_to_u8_array(first_sample_flags);
            data = [
                data,
                vec![array_val[3], array_val[2], array_val[1], array_val[0]],
            ]
            .concat()
        }

        // Check for sample metadata present if the samples array has more than 1 sample. The first sample will most likely utilize
        // the first sample flag.
        if self.samples.len() > 1 {
            println!("SAMPLES");
            flag += 0x000200; // Each sample info contains the u8 array of the sample so we will always have the sample size.
            let sample_info = &self.samples[1];
            // If sample composition time offsets is present, we just need this, else just use duration
            println!("sample_composition_time_offsets_present: {}", self.sample_composition_time_offsets_present);
            if self.sample_composition_time_offsets_present {
                flag += 0x000800;
            } 
            
            if let Some(_) = sample_info.sample_duration {
                flag += 0x000100;
            }

            if let Some(_) = sample_info.sample_flags {
                flag += 0x000400;
            }
        }
        (flag, data)
    }

    pub fn build(&self) -> Vec<u8> {
        let version_array = util::transform_usize_to_u8_array(self.version);
        // let flags_array = util::transform_usize_to_u8_array(self.flags);
         let (flags, data) = self.generate_flag();
        let flag_array = util::transform_u32_to_u8_array(flags);
        let sample_count_array = util::transform_usize_to_u8_array(self.samples.len());
        let calculated_sample_size = self.calculate_sample_size(flags);
        let all_samples_size = calculated_sample_size * self.samples.len();
        let sample_data = TRUNBuilder::create_sample_data(
            &self.samples,
            calculated_sample_size,
            flags,
            self.version,
        );
       

        let mut size: usize = 12 + // header
        4 + // sample_count
        4 + // data_offset. NOTE(benjamintoofer@gmail.com): This is optional but for CMAF it's required...We doin CMAF son 
        data.len(); // Basically if first-sample-flags-present was set or not.

        // if let Some(_) = self.first_sample_flags {
        //     size += 4;
        // }

        size += all_samples_size;
        let size_array = util::transform_usize_to_u8_array(size);
        let final_data_offset = size + self.data_offset + 8; // mdat header
        let data_offset_array = util::transform_usize_to_u8_array(final_data_offset);

        let trun = 
        [
            vec![
                // Size
                size_array[3],size_array[2],size_array[1],size_array[0],
                // trun
                0x74, 0x72, 0x75, 0x6E,
                // version
                version_array[0],
                // flag
                flag_array[2], flag_array[1], flag_array[0],
                // sample_count
                sample_count_array[3],sample_count_array[2],sample_count_array[1],sample_count_array[0],
                // data_offset (optional but it is required for CMAF)
                data_offset_array[3],data_offset_array[2],data_offset_array[1],data_offset_array[0],
            ],
            data,
        ].concat();
        
        // if let Some(first_sample_flags) = self.first_sample_flags {
        //     let first_sample_flags_array =
        //         util::transform_usize_to_u8_array(first_sample_flags).to_vec();
        //     // first_sample_flags_array.reverse();
        //     trun = [
        //         trun,
        //         vec![
        //             first_sample_flags_array[3],
        //             first_sample_flags_array[2],
        //             first_sample_flags_array[1],
        //             first_sample_flags_array[0],
        //         ],
        //     ]
        //     .concat();
        // }

        [
            trun,
            sample_data
        ].concat()
    }

    fn calculate_sample_size(&self, flags: u32) -> usize {
        let mut calc_size = 0usize;
        if flags & 0x000100 != 0 {
            // sample-duration-present
            calc_size += 4;
        }
        if flags & 0x000200 != 0 {
            // sample-size-present
            calc_size += 4;
        }
        if flags & 0x000400 != 0 {
            // sample-flags-present
            calc_size += 4;
        }
        if flags & 0x000800 != 0 {
            // sample-composition-time-offsets-present
            calc_size += 4;
        }
        calc_size
    }

    fn create_sample_data(
        samples: &Vec<SampleInfo>,
        sample_size: usize,
        flags: u32,
        version: usize,
    ) -> Vec<u8> {
        let total_sample_size = samples.len() * sample_size;
        let mut data: Vec<u8> = vec![0; total_sample_size];
        let mut offset = 0usize;
        for sample_info in samples.iter() {
            let duration = sample_info.sample_duration.unwrap_or_default();
            let sample = TRUNBuilder::create_sample(sample_info, sample_size, flags, version, duration, 0);
            let end = offset + sample_size;
            data.splice(offset..end, sample);
            offset = end;
        }

        data
    }

    pub fn create_sample(
        sample: &SampleInfo,
        sample_size: usize,
        flags: u32,
        version: usize,
        duration: u32,
        sample_flag: usize,
    ) -> Vec<u8> {
        let mut sample_data = vec![0u8; sample_size];
        let mut offset = 0usize;
        if flags & 0x000100 != 0 {
            // sample-duration-present
            let duration_array = util::transform_u32_to_u8_array(duration);
            let end = offset + 4;
            sample_data.splice(
                offset..end,
                vec![
                    duration_array[3], duration_array[2], duration_array[1], duration_array[0],
                ],
            );
            offset = end;
        }
        if flags & 0x000200 != 0 {
            // sample-size-present
            let size_array = util::transform_usize_to_u8_array(sample.data.len());
            let end = offset + 4;
            sample_data.splice(
                offset..end,
                vec![size_array[3], size_array[2], size_array[1], size_array[0]],
            );
            offset = end;
        }
        if flags & 0x000400 != 0 {
            // sample-flags-present
            let sf_array = util::transform_usize_to_u8_array(sample_flag);
            let end = offset + 4;
            sample_data.splice(
                offset..end,
                vec![sf_array[3], sf_array[2], sf_array[1], sf_array[0]],
            );
            offset = end;
        }
        if flags & 0x000800 != 0 {
            // sample-composition-time-offsets-present
            println!("HERE!");
            let data: Vec<u8>;
            if version == 1 {
                let diff = sample.pts as i32 - sample.dts as i32;
                println!("DIFF -- 1 -- : {}", diff);
                data = util::transform_i32_to_u8_array(diff).to_vec();
            } else {
                let diff = (sample.pts - sample.dts) as usize;
                println!("DIFF -- 0 -- : {}", diff);
                data = util::transform_usize_to_u8_array(diff).to_vec();
            }
            let end = offset + 4;
            sample_data.splice(offset..end, vec![data[3], data[2], data[1], data[0]]);
        }

        sample_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_trun() {
        let trun: [u8; 384] = [
            // size
            0x00, 0x00, 0x01, 0x80, // trun
            0x74, 0x72, 0x75, 0x6E, 0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x00, 0x5A, 0x00, 0x00,
            0x01, 0xD8, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0xAC, 0x00, 0x00, 0x01, 0x14,
            0x00, 0x00, 0x00, 0xDB, 0x00, 0x00, 0x01, 0x7E, 0x00, 0x00, 0x01, 0xBE, 0x00, 0x00,
            0x01, 0xF6, 0x00, 0x00, 0x02, 0x5E, 0x00, 0x00, 0x02, 0x84, 0x00, 0x00, 0x02, 0x02,
            0x00, 0x00, 0x02, 0x8D, 0x00, 0x00, 0x02, 0xC6, 0x00, 0x00, 0x02, 0x5E, 0x00, 0x00,
            0x02, 0xBC, 0x00, 0x00, 0x02, 0xB9, 0x00, 0x00, 0x02, 0xDE, 0x00, 0x00, 0x02, 0x94,
            0x00, 0x00, 0x02, 0xB1, 0x00, 0x00, 0x02, 0xE3, 0x00, 0x00, 0x02, 0xF4, 0x00, 0x00,
            0x02, 0x5A, 0x00, 0x00, 0x02, 0xD9, 0x00, 0x00, 0x02, 0x89, 0x00, 0x00, 0x02, 0xBD,
            0x00, 0x00, 0x02, 0xBA, 0x00, 0x00, 0x03, 0x4C, 0x00, 0x00, 0x02, 0x9B, 0x00, 0x00,
            0x02, 0xFE, 0x00, 0x00, 0x03, 0x11, 0x00, 0x00, 0x02, 0xD3, 0x00, 0x00, 0x03, 0x69,
            0x00, 0x00, 0x02, 0x8E, 0x00, 0x00, 0x02, 0xE4, 0x00, 0x00, 0x02, 0x5B, 0x00, 0x00,
            0x02, 0xFB, 0x00, 0x00, 0x03, 0x31, 0x00, 0x00, 0x03, 0x23, 0x00, 0x00, 0x05, 0x04,
            0x00, 0x00, 0x04, 0x95, 0x00, 0x00, 0x05, 0x55, 0x00, 0x00, 0x05, 0x09, 0x00, 0x00,
            0x05, 0x34, 0x00, 0x00, 0x04, 0xD8, 0x00, 0x00, 0x05, 0x12, 0x00, 0x00, 0x05, 0x8B,
            0x00, 0x00, 0x04, 0xBD, 0x00, 0x00, 0x05, 0x54, 0x00, 0x00, 0x04, 0xF5, 0x00, 0x00,
            0x04, 0xE1, 0x00, 0x00, 0x05, 0x47, 0x00, 0x00, 0x05, 0xB2, 0x00, 0x00, 0x04, 0x62,
            0x00, 0x00, 0x04, 0x26, 0x00, 0x00, 0x03, 0xFC, 0x00, 0x00, 0x03, 0xBF, 0x00, 0x00,
            0x03, 0x68, 0x00, 0x00, 0x03, 0x8E, 0x00, 0x00, 0x04, 0x46, 0x00, 0x00, 0x06, 0x48,
            0x00, 0x00, 0x05, 0xE9, 0x00, 0x00, 0x05, 0x2D, 0x00, 0x00, 0x05, 0x6D, 0x00, 0x00,
            0x04, 0x7C, 0x00, 0x00, 0x04, 0x93, 0x00, 0x00, 0x04, 0x9B, 0x00, 0x00, 0x04, 0xEE,
            0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x04, 0xDC, 0x00, 0x00, 0x04, 0xC8, 0x00, 0x00,
            0x04, 0x9F, 0x00, 0x00, 0x04, 0x87, 0x00, 0x00, 0x04, 0xA6, 0x00, 0x00, 0x04, 0x9F,
            0x00, 0x00, 0x04, 0x67, 0x00, 0x00, 0x04, 0x58, 0x00, 0x00, 0x04, 0x65, 0x00, 0x00,
            0x04, 0x8F, 0x00, 0x00, 0x04, 0x71, 0x00, 0x00, 0x05, 0x69, 0x00, 0x00, 0x05, 0x67,
            0x00, 0x00, 0x05, 0x89, 0x00, 0x00, 0x05, 0x86, 0x00, 0x00, 0x05, 0xCD, 0x00, 0x00,
            0x05, 0x03, 0x00, 0x00, 0x05, 0x32, 0x00, 0x00, 0x05, 0x58, 0x00, 0x00, 0x05, 0x30,
            0x00, 0x00, 0x05, 0x07, 0x00, 0x00, 0x04, 0xDF, 0x00, 0x00, 0x05, 0x0E, 0x00, 0x00,
            0x05, 0x11,
        ];

        let expected_trun: TRUN = TRUN {
            box_type: "trun".to_string(),
            size: 384,
            version: 0,
            flags: 0x205,
            data_offset: Option::Some(472),
            first_sample_flags: Option::Some(0x2000000),
            sample_count: 90,
            samples: vec![
                Sample {
                    sample_size: Option::Some(13740),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(276),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(219),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(382),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(446),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(502),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(606),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(644),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(514),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(653),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(710),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(606),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(700),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(697),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(734),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(660),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(689),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(739),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(756),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(602),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(729),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(649),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(701),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(698),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(844),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(667),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(766),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(785),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(723),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(873),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(654),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(740),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(603),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(763),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(817),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(803),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1284),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1173),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1365),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1289),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1332),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1240),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1298),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1419),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1213),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1364),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1269),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1249),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1351),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1458),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1122),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1062),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1020),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(959),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(872),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(910),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1094),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1608),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1513),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1325),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1389),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1148),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1171),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1179),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1262),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1152),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1244),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1224),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1183),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1127),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1112),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1125),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1167),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1137),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1385),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1383),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1417),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1414),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1485),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1283),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1330),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1368),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1328),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1287),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1247),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1294),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
                Sample {
                    sample_size: Option::Some(1297),
                    sample_duration: Option::None,
                    sample_flags: Option::None,
                    sample_composition_time_offset: None,
                },
            ],
        };
        assert_eq!(TRUN::parse_trun(&trun).unwrap(), expected_trun);
    }

    #[test]
    fn test_build_trun() {
        let expected_trun: [u8; 384] = [
            // size
            0x00, 0x00, 0x01, 0x80, // trun
            0x74, 0x72, 0x75, 0x6E, 0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x00, 0x5A, 0x00, 0x00,
            0x01, 0xE0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x46, 0x00, 0x00, 0x01, 0xEB,
            0x00, 0x00, 0x02, 0xD8, 0x00, 0x00, 0x02, 0xF7, 0x00, 0x00, 0x04, 0x03, 0x00, 0x00,
            0x02, 0xF0, 0x00, 0x00, 0x03, 0x9B, 0x00, 0x00, 0x03, 0xC2, 0x00, 0x00, 0x03, 0xFD,
            0x00, 0x00, 0x04, 0x2D, 0x00, 0x00, 0x04, 0x0E, 0x00, 0x00, 0x03, 0xDF, 0x00, 0x00,
            0x03, 0xB6, 0x00, 0x00, 0x03, 0xD7, 0x00, 0x00, 0x04, 0x19, 0x00, 0x00, 0x04, 0x52,
            0x00, 0x00, 0x04, 0xC8, 0x00, 0x00, 0x04, 0x9D, 0x00, 0x00, 0x04, 0xBF, 0x00, 0x00,
            0x04, 0x5F, 0x00, 0x00, 0x04, 0x94, 0x00, 0x00, 0x04, 0xEB, 0x00, 0x00, 0x05, 0x26,
            0x00, 0x00, 0x05, 0x06, 0x00, 0x00, 0x04, 0xE4, 0x00, 0x00, 0x04, 0x58, 0x00, 0x00,
            0x04, 0xB1, 0x00, 0x00, 0x04, 0x83, 0x00, 0x00, 0x04, 0xB5, 0x00, 0x00, 0x04, 0xAE,
            0x00, 0x00, 0x05, 0x3F, 0x00, 0x00, 0x04, 0x8C, 0x00, 0x00, 0x05, 0x37, 0x00, 0x00,
            0x05, 0xF4, 0x00, 0x00, 0x05, 0x73, 0x00, 0x00, 0x04, 0xF4, 0x00, 0x00, 0x05, 0x5C,
            0x00, 0x00, 0x04, 0x9E, 0x00, 0x00, 0x05, 0x01, 0x00, 0x00, 0x04, 0xAF, 0x00, 0x00,
            0x05, 0x20, 0x00, 0x00, 0x04, 0xDE, 0x00, 0x00, 0x05, 0xA1, 0x00, 0x00, 0x05, 0x27,
            0x00, 0x00, 0x04, 0xCE, 0x00, 0x00, 0x04, 0xED, 0x00, 0x00, 0x04, 0xDD, 0x00, 0x00,
            0x04, 0xE1, 0x00, 0x00, 0x05, 0x9C, 0x00, 0x00, 0x05, 0x1B, 0x00, 0x00, 0x04, 0xF7,
            0x00, 0x00, 0x04, 0xBB, 0x00, 0x00, 0x04, 0x87, 0x00, 0x00, 0x04, 0xBF, 0x00, 0x00,
            0x04, 0xEF, 0x00, 0x00, 0x04, 0xE1, 0x00, 0x00, 0x04, 0xC0, 0x00, 0x00, 0x05, 0x07,
            0x00, 0x00, 0x05, 0x1B, 0x00, 0x00, 0x04, 0xD9, 0x00, 0x00, 0x05, 0x02, 0x00, 0x00,
            0x04, 0x5E, 0x00, 0x00, 0x04, 0xD8, 0x00, 0x00, 0x04, 0xAE, 0x00, 0x00, 0x04, 0xFA,
            0x00, 0x00, 0x04, 0xC2, 0x00, 0x00, 0x04, 0xA4, 0x00, 0x00, 0x05, 0x29, 0x00, 0x00,
            0x05, 0x27, 0x00, 0x00, 0x04, 0xBE, 0x00, 0x00, 0x05, 0x0A, 0x00, 0x00, 0x04, 0xA2,
            0x00, 0x00, 0x04, 0x59, 0x00, 0x00, 0x05, 0x2C, 0x00, 0x00, 0x04, 0xEE, 0x00, 0x00,
            0x04, 0x6A, 0x00, 0x00, 0x04, 0xDA, 0x00, 0x00, 0x04, 0xF4, 0x00, 0x00, 0x04, 0xE4,
            0x00, 0x00, 0x04, 0x30, 0x00, 0x00, 0x04, 0xF2, 0x00, 0x00, 0x04, 0x67, 0x00, 0x00,
            0x04, 0x6D, 0x00, 0x00, 0x05, 0x4E, 0x00, 0x00, 0x04, 0xF0, 0x00, 0x00, 0x05, 0x72,
            0x00, 0x00, 0x04, 0xAB, 0x00, 0x00, 0x05, 0x01, 0x00, 0x00, 0x04, 0xE6, 0x00, 0x00,
            0x05, 0x06,
        ];

        let nal_units = generate_test_nal_units();

        let trun = TRUNBuilder::create_builder()
            .version(0)
            .flags(0x0205)
            .first_sample_flags(0x2000000)
            .data_offset(88)
            .samples(nal_units)
            .build();
        assert_eq!(trun, expected_trun);
    }

    #[test]
    fn test_create_sample() {
        let sample = SampleInfo {
            sample_flags: None,
            sample_duration: None,
            data: vec![0x00, 0x01, 0x02, 0x03, 0x04],
            pts: 0,
            dts: 0,
        };
        let sample_size = 16;
        let flags = 0x000F00;
        TRUNBuilder::create_sample(&sample, sample_size, flags, 0, 0, 0);
        // TODO (benjamintoofer@gmail.com): Finish this unit test
    }

    fn generate_test_nal_units() -> Vec<SampleInfo> {
        vec![
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 14150],
                pts: 0,
                dts: 0,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 491],
                pts: 1,
                dts: 1,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 728],
                pts: 2,
                dts: 2,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 759],
                pts: 3,
                dts: 3,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1027],
                pts: 4,
                dts: 4,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 752],
                pts: 5,
                dts: 5,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 923],
                pts: 6,
                dts: 6,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 962],
                pts: 7,
                dts: 7,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1021],
                pts: 8,
                dts: 8,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1069],
                pts: 9,
                dts: 9,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1038],
                pts: 10,
                dts: 10,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 991],
                pts: 11,
                dts: 11,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 950],
                pts: 12,
                dts: 12,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 983],
                pts: 13,
                dts: 13,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1049],
                pts: 14,
                dts: 14,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1106],
                pts: 15,
                dts: 15,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1224],
                pts: 16,
                dts: 16,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1181],
                pts: 17,
                dts: 17,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1215],
                pts: 18,
                dts: 18,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1119],
                pts: 19,
                dts: 19,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1172],
                pts: 20,
                dts: 20,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1259],
                pts: 21,
                dts: 21,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1318],
                pts: 22,
                dts: 22,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1286],
                pts: 23,
                dts: 23,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1252],
                pts: 24,
                dts: 24,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1112],
                pts: 25,
                dts: 25,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1201],
                pts: 26,
                dts: 26,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1155],
                pts: 27,
                dts: 27,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1205],
                pts: 28,
                dts: 28,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1198],
                pts: 29,
                dts: 29,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1343],
                pts: 30,
                dts: 30,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1164],
                pts: 31,
                dts: 31,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1335],
                pts: 32,
                dts: 32,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1524],
                pts: 33,
                dts: 33,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1395],
                pts: 34,
                dts: 34,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1268],
                pts: 35,
                dts: 35,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1372],
                pts: 36,
                dts: 36,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1182],
                pts: 37,
                dts: 37,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1281],
                pts: 38,
                dts: 38,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1199],
                pts: 39,
                dts: 39,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1312],
                pts: 40,
                dts: 40,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1246],
                pts: 41,
                dts: 41,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1441],
                pts: 42,
                dts: 42,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1319],
                pts: 43,
                dts: 43,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1230],
                pts: 44,
                dts: 44,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1261],
                pts: 45,
                dts: 45,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1245],
                pts: 46,
                dts: 46,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1249],
                pts: 47,
                dts: 47,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1436],
                pts: 48,
                dts: 48,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1307],
                pts: 49,
                dts: 49,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1271],
                pts: 50,
                dts: 50,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1211],
                pts: 51,
                dts: 51,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1159],
                pts: 52,
                dts: 52,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1215],
                pts: 53,
                dts: 53,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1263],
                pts: 54,
                dts: 54,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1249],
                pts: 55,
                dts: 55,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1216],
                pts: 56,
                dts: 56,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1287],
                pts: 57,
                dts: 57,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1307],
                pts: 58,
                dts: 58,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1241],
                pts: 59,
                dts: 59,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1282],
                pts: 60,
                dts: 60,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1118],
                pts: 61,
                dts: 61,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1240],
                pts: 62,
                dts: 62,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1198],
                pts: 63,
                dts: 63,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1274],
                pts: 64,
                dts: 64,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1218],
                pts: 65,
                dts: 65,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1188],
                pts: 66,
                dts: 66,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1321],
                pts: 67,
                dts: 67,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1319],
                pts: 68,
                dts: 68,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1214],
                pts: 69,
                dts: 69,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1290],
                pts: 70,
                dts: 70,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1186],
                pts: 71,
                dts: 71,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1113],
                pts: 72,
                dts: 72,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1324],
                pts: 73,
                dts: 73,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1262],
                pts: 74,
                dts: 74,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1130],
                pts: 75,
                dts: 75,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1242],
                pts: 76,
                dts: 76,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1268],
                pts: 77,
                dts: 77,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1252],
                pts: 78,
                dts: 78,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1072],
                pts: 79,
                dts: 79,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1266],
                pts: 80,
                dts: 80,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1127],
                pts: 81,
                dts: 81,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1133],
                pts: 82,
                dts: 82,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1358],
                pts: 83,
                dts: 83,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1264],
                pts: 84,
                dts: 84,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1394],
                pts: 85,
                dts: 85,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1195],
                pts: 86,
                dts: 86,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1281],
                pts: 87,
                dts: 87,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1254],
                pts: 88,
                dts: 88,
            },
            SampleInfo {
                sample_flags: None,
                sample_duration: None,
                data: vec![0; 1286],
                pts: 89,
                dts: 89,
            },
        ]
    }
}
