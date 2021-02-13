use crate::util;

static CLASS: &str = "AudioSampleEntry";
#[derive(Debug)]
pub struct AudioSampleEntry {
  channel_count: u16,
  sample_size: u16,
  sample_rate: u32,
}

impl AudioSampleEntry {
  pub fn parse(data: &[u8]) -> (AudioSampleEntry, usize) {
    let offset = 24usize;
    let mut start = offset;
    let channel_count = util::get_u16(data, start)
      .expect(format!("{}.parse.channel_count: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 2;
    // Parse sample size
    let sample_size = util::get_u16(data, start)
      .expect(format!("{}.parse.sample_size: cannot get u16 from start = {}",CLASS, start).as_ref());

    start = start + 6;
    // Parse sample size
    let sample_rate = util::get_u32(data, start)
      .and_then(|val| Ok(val >> 16u32))
      .expect(format!("{}.parse.sample_rate: cannot get u32 from start = {}",CLASS, start).as_ref());


    (AudioSampleEntry {
      channel_count,
      sample_size,
      sample_rate
    }, start + 4)
  }
}
