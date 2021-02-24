
pub struct PESPacket {
  packet_start_code_prefix: u32,        // 24 bit
  stream_id: u8,
  PES_packet_length: u16,
}