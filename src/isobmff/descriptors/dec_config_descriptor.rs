use crate::util;

static CLASS: &str = "DecoderConfigDescriptor";
#[derive(Debug)]
pub struct DecoderConfigDescriptor {
  object_type_indication: u8,
  stream_type: u8,            // 6 bit
  upstream: bool,             // 1 bit
  buffer_size_db: u32,         // 24 bit
  max_bitrate: u32,
  avg_bitrate: u32,
}

impl  DecoderConfigDescriptor {
  pub fn parse(data: &[u8]) -> DecoderConfigDescriptor {
    let mut start = 2usize;
    let mut end = start + 1;
    // Parse object_type_indication
    let object_type_indication = util::get_u8(data, start, end)
      .expect(format!("{}.parse.object_type_indication: cannot get u8 from start = {}; end = {}",CLASS, start, end).as_ref());

    start = end;
    end = start + 1;
    let temp = util::get_u8(data, start, end)
      .expect(format!("{}.parse.temp: cannot get u8 from start = {}; end = {}",CLASS, start, end).as_ref());
    let stream_type = (temp & 0xFC) >> 2;
    let upstream = (temp & 0x2) != 0;

    let mut buffer_size_db: u32 = 0;
    for i in 0..3 {
      println!("{}", i);
      start = end;
      end = start + 1;
      let buff = util::get_u8(data, start, end)
        .expect(format!("{}.parse.temp: cannot get u8 from start = {}; end = {}",CLASS, start, end).as_ref());
        buffer_size_db =  buffer_size_db | (u32::from(buff) << (8 * (2 - i)));
    }

    start = end;
    end = start + 4;
    let max_bitrate = util::get_u32(data, start, end)
      .expect(format!("{}.parse.temp: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());

    start = end;
    end = start + 4;
    let avg_bitrate = util::get_u32(data, start, end)
      .expect(format!("{}.parse.avg_bitrate: cannot get u32 from start = {}; end = {}",CLASS, start, end).as_ref());

    
    DecoderConfigDescriptor {
      object_type_indication,
      stream_type,
      upstream,
      buffer_size_db,
      max_bitrate,
      avg_bitrate,
    }
  }
}