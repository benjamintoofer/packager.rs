use crate::error::{CustomError, construct_error, error_code::{MajorCode, TransportStreamMinorCode}};
use crate::util::bit_reader::BitReader;

pub struct ADTSHeader {
  id_version: u8,
  profile: u8,
  sampling_frequency_index: u8,
  channel_configuration: u8,
  frame_length: u16,
  crc: Option<u16>
}

pub struct ADTSFrame {
  header: ADTSHeader,
  data: Vec<u8>,
}


#[derive(Debug)]
pub struct ADTS {
  id_version: u8,
  profile: u8,
  sampling_frequency_index: u8,
  channel_configuration: u8,
  crc: Option<u16>
}

impl ADTS {
  pub fn parse(data: &[u8]) -> Result<Vec<ADTSFrame>, CustomError> {
    let mut data_read = data;
    let mut index = 0usize;
    while data_read.len() > 0 {
      let adts_header = ADTS::parse_adts_header(data_read)?;
      let start = index;
      let end = start + adts_header.frame_length as usize;
      data_read = data_read[start..end].as_ref();

      index = end;
    }
    Ok(
      vec![]
    )
  }

  fn parse_adts_header(data: &[u8]) -> Result<ADTSHeader, CustomError> {
    let mut bit_reader = BitReader::create_bit_reader(data);
    let starting_marker = bit_reader.read_bits(16)?;

    let has_start_marker = (starting_marker & 0xFFF0) == 0xFFF0;
    if !has_start_marker {
      return Err(
        construct_error(
          MajorCode::TRANSPORT_STREAM,
          Box::new(TransportStreamMinorCode::PARSE_TS_ERROR),
          format!("Could not parse adts starting marker"),
          file!(),
          line!()
        )
      );
    }
    let temp = starting_marker & 0xF;
    let id_version = (temp & 0x8) as u8;
    let protection_absent = temp & 0x1 == 0;
    let profile = bit_reader.read_bits(2)? as u8;
    let sampling_frequency_index = bit_reader.read_bits(4)? as u8;
    // Skip private bit
    bit_reader.read_bits(1)?;
    let channel_configuration = bit_reader.read_bits(3)? as u8;
    // Skip original_copy, home, copyright_identification_bit, and copyright_identification_start
    bit_reader.read_bits(4)?;
    let frame_length = bit_reader.read_bits(13)? as u16;
    let buffer_fullness = bit_reader.read_bits(11)?;
    let number_of_raw_data_blocks_in_frame = bit_reader.read_bits(2)?;

    // Checkout ISO/IEC 13818-7: 6.2 Audio Data Transport Stream, ADTS :: Table 5 — Syntax of adts_frame()
    // If there are more than 1 frame per data block, we need to handle it. I think todays expectation is to
    // only have 1 aac frame per adts frame
    if number_of_raw_data_blocks_in_frame > 0 {
      return Err(
        construct_error(
          MajorCode::TRANSPORT_STREAM,
          Box::new(TransportStreamMinorCode::UNSUPPORTED_ADTS_PARSING),
          format!("Could not parse because there is more than 1 aac frame in adtc frame. This is not supported in the current implementation."),
          file!(),
          line!()
        )
      );
    }
    let mut crc: Option<u16> = None;
    if protection_absent {
      crc = Some(bit_reader.read_bits(16)? as u16);
    }
  
    println!("----\nProfile: {}\nSample Freq: {}\nChannel Config: {}\nFrame Length: {}\nBuffer Fullness: {}\nNum Raw Data Blocks in Frame: {}\n-------", profile, sampling_frequency_index, channel_configuration, frame_length, buffer_fullness, number_of_raw_data_blocks_in_frame);
    println!("TOTAL SIZE: {}", data.len());

    Ok(
      ADTSHeader{
        id_version,
        profile,
        sampling_frequency_index,
        channel_configuration,
        frame_length,
        crc,
      }
    )
  }
}