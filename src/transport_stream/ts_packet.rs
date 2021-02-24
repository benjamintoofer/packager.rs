use crate::error::{CustomError};
use crate::util;

static TS_PACKET_SIZE: usize = 188;


#[derive(Debug, Eq)]
pub struct TransportPacket<'a> {
  transport_error_indicator: bool,
  pub payload_unit_start_indicator: bool,
  transport_priority: bool,
  pub pid: u16,                         // 13 bit
  transport_scrambling_control: u8,     // 2 bit
  pub adaptation_field_control: u8,     // 2 bit
  continuity_counter: u8,               // 4 bit
  pub data: &'a [u8]
}

impl<'a> PartialEq for TransportPacket<'a> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a> TransportPacket<'a> {
  pub fn parse(ts: &[u8]) -> Result<TransportPacket, CustomError> {
    TransportPacket::parse_transport_packet(ts)
  }

  fn parse_transport_packet(ts: &[u8]) -> Result<TransportPacket, CustomError> {
    let mut start = 1usize;

    let indicator_data = util::get_u16(ts, start)?;
    let transport_error_indicator = (indicator_data & 0x8000) != 0;
    let payload_unit_start_indicator = (indicator_data & 0x4000) != 0;
    let transport_priority = (indicator_data & 0x2000) != 0;
    let pid = indicator_data & 0x1FFF;

    if pid < 257 {
      println!("PID - {}", pid);
    }
    start = start + 2;
    let control_data = util::get_u8(ts, start)?;
    let transport_scrambling_control = (control_data & 0xC0) >> 6;
    let adaptation_field_control = (control_data & 0x30) >> 4;
    let continuity_counter = control_data & 0xF;
    start = start + 1;

    let mut adaptation_field_length = 0usize;
    if adaptation_field_control > 1 {
      adaptation_field_length = util::get_u8(ts, start)? as usize;
      start = start + 1;
    }

    start = start + adaptation_field_length;
    if start == TS_PACKET_SIZE {
      // Only an adaptation field. Just return nothing
      return Ok(TransportPacket {
        transport_error_indicator,
        payload_unit_start_indicator,
        transport_priority,
        pid,
        transport_scrambling_control,
        adaptation_field_control,
        continuity_counter,
        data: &[]
      })
    }

    Ok(TransportPacket {
      transport_error_indicator,
      payload_unit_start_indicator,
      transport_priority,
      pid,
      transport_scrambling_control,
      adaptation_field_control,
      continuity_counter,
      data: ts[start..TS_PACKET_SIZE].as_ref()
    })
  }
}