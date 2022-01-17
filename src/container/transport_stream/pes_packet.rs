use crate::error::CustomError;
use crate::util;
use crate::error::error_code::{MajorCode, TransportStreamMinorCode};
use crate::error::construct_error;

static PACKET_START_CODE_PREFIX: u32 = 0x000001;
pub struct PESPacket<'a> {
  pub packet_start_code_prefix: u32,        // 24 bit
  pub stream_id: u8,
  pub pes_packet_length: u16,
  pub payload_data: &'a [u8],
  pub dts: Option<u64>,
  pub pts: Option<u64>,
}

impl<'a> PESPacket<'a> {
  pub fn parse(payload: &[u8]) -> Result<PESPacket, CustomError> {
    PESPacket::parse_pes_packet(payload)
  }

  fn parse_pes_packet(payload: &[u8]) -> Result<PESPacket, CustomError> {
    let mut offset = 0usize;
    // PES header is 19 bytes. Could be a chance that the paload is under 19 bytes, if so need to implement storing this, and
    // wait for the next ts packet to merge these two pes packets
    if payload.len() < 19 {
      return Ok(PESPacket{
        packet_start_code_prefix: 0,
        stream_id: 0,
        pes_packet_length: 0,
        payload_data: payload,
        dts: None,
        pts: None,
      })
    }
    let data = util::get_u32(payload, offset)?;
    offset += 4;
    let start_prefix = (data & 0xFFFFFF00) >> 8;
    if start_prefix != PACKET_START_CODE_PREFIX {
      return Ok(PESPacket{
        packet_start_code_prefix: 0,
        stream_id: 0,
        pes_packet_length: 0,
        payload_data: payload,
        dts: None,
        pts: None,
      });
    }
    let stream_id = (data & 0xFF) as u8;
    let pes_packet_length = util::get_u16(payload, offset)?;
    // Skip 1 byte + 2 bytes for packet length
    offset += 3;
    
    if
      (stream_id >> 4) == 0b1110 || // Checking if ISO/IEC 13818-3 or ISO/IEC 11172-3 or ISO/IEC 13818-7 or ISO/IEC 14496-3 audio stream number x xxxx
      (stream_id >> 5) == 0b110 // Checking if ITU-T Rec. H.262 | ISO/IEC 13818-2 or ISO/IEC 11172-2 or ISO/IEC 14496-2 video stream number xxxx
    {
      let pts_dts_flags = util::get_u8(payload, offset)?;
      offset += 1;
      let pes_header_data_length = util::get_u8(payload, offset)?;
      offset += 1;
      let mut pts = 0;
      let mut dts = u64::MAX;
      if (pts_dts_flags & 0xC0) != 0 {
        let pts_section_64 = util::get_u64(payload, offset)?;
        // Only 40 bits are being used.
        offset += 5;
        let pts_section = pts_section_64 >> 24;
        if !PESPacket::is_timestamp_section_valid(pts_section) {
          println!("Not a valid PTS");
        }
        pts = PESPacket::convert_timestamp_section_to_timesamp(pts_section);
      }

      if (pts_dts_flags & 0x40) != 0 {
        let dts_section_64 = util::get_u64(payload, offset)?;
        // Only 40 bits are being used.
        offset += 5;
        let dts_section = dts_section_64 >> 24;
        if !PESPacket::is_timestamp_section_valid(dts_section) {
          println!("Not a valid DTS");
        }
        dts = PESPacket::convert_timestamp_section_to_timesamp(dts_section);
      }

      if dts == u64::MAX {
        dts = pts
      }
      // 9 bytes : 6 bytes for PES header + 3 bytes for PES extension
      let payload_start_offset = (pes_header_data_length + 9) as usize;
      return Ok(PESPacket{
        packet_start_code_prefix: start_prefix,
        stream_id,
        pes_packet_length: 0,
        payload_data: payload[payload_start_offset..payload.len()].as_ref(),
        dts: Some(dts),
        pts: Some(pts),
      });
    }
    
    return Err(construct_error(
        MajorCode::TRANSPORT_STREAM, 
        Box::new(TransportStreamMinorCode::PARSE_TS_ERROR),
        "Stream id is not valid".to_string(), 
        file!(), 
        line!()));
  }

  // https://github.com/google/shaka-packager/blob/6c8ad30217c286d4eecadc9df12420767d389942/packager/media/formats/mp2t/ts_section_pes.cc#L63-L73
  fn is_timestamp_section_valid(timestamp_section: u64) -> bool {
    return ((timestamp_section & 0x1) != 0) &&
         ((timestamp_section & 0x10000) != 0) &&
         ((timestamp_section & 0x100000000u64) != 0);
  }

  // https://github.com/google/shaka-packager/blob/6c8ad30217c286d4eecadc9df12420767d389942/packager/media/formats/mp2t/ts_section_pes.cc#L75-L79
  fn convert_timestamp_section_to_timesamp(timestamp_section: u64) -> u64 {
    return (((timestamp_section >> 33) & 0x7) << 30) |
         (((timestamp_section >> 17) & 0x7fff) << 15) |
         (((timestamp_section >> 1) & 0x7fff) << 0);
  }
}