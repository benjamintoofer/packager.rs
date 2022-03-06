use crate::{container::{isobmff::nal::NalRep, transport_stream::pes_packet}, error::CustomError};
use crate::container::isobmff::nal::{nal_unit::NALUnit, NALType};


pub struct AVCExtractor<IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<NalRep>),
{
  sps_nal: Vec<u8>,
  pps_nal: Vec<u8>,
  media_nal: Vec<NalRep>,
  bucket: Vec<u8>,
  init_callback: Option<IF>,
  media_callback: Option<MF>,
  signed_comp_offset: bool,
  all_same_timestamps: bool,
  current_pts: u64,
  current_dts: u64,
}

impl<IF, MF> AVCExtractor<IF, MF>
where
  IF: Fn(&Vec<u8>, &Vec<u8>),
  MF: Fn(&Vec<NalRep>),
{
  fn create() -> AVCExtractor<IF, MF> {
    AVCExtractor {
      sps_nal: vec![],
      pps_nal: vec![],
      media_nal: vec![],
      bucket: vec![],
      init_callback: None,
      media_callback: None,
      all_same_timestamps: true,
      signed_comp_offset: false,
      current_pts: 0,
      current_dts: 0,
    }
  }

  fn is_all_same_timestamps(self) -> bool {
    self.all_same_timestamps
  }

  fn is_signed_comp_offset(self) -> bool {
    self.signed_comp_offset
  }

  fn accumulate_pes_payload(&mut self, pes: pes_packet::PESPacket) -> Result<(), CustomError> {
    let mut index: usize = 0;
    let mut nal_start_index = index;
    let pes_payload = pes.payload_data;

    loop {
      if (index + 4) >= pes_payload.len() { // If the next 4 bytes is greater than the total payload, add it to the bucket for the next pes payload
          let mut leftover: Vec<u8> = pes_payload[nal_start_index..pes_payload.len()].to_vec();
          self.bucket.append(&mut leftover);
          break;
      }
      let boundary = NALUnit::find_boundary(index, pes_payload);
      if boundary == -1 { // If no nal boundary is found, increment the index and continue searching for the next boundary
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
        // Have the data to create the init segment
        if self.sps_nal.len() > 0 && self.pps_nal.len() > 0 {
            if let Some(cb) = &self.init_callback {
                let sps = self.sps_nal[0..].to_vec();
                let pps = self.pps_nal[0..].to_vec();
                self.sps_nal.clear();
                self.pps_nal.clear();
                cb(&sps, &pps);
            }
        }
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
      if pts > dts {
        self.all_same_timestamps = false;
      }

      self.current_pts = pts;
      self.current_dts = dts;
    }

    Ok(())
  }

  fn listen_for_init_data(&mut self, callback: IF) -> &Self {
      self.init_callback = Some(callback);
      return self;
  }

  fn listen_for_media_data(&mut self, callback: MF) -> &Self {
      self.media_callback = Some(callback);
      return self;
  }

  fn handle_nal_unit(&mut self, nal_type: NALType, nal_unit: &[u8]) {
    match nal_type {
      NALType::SPS => {
        self.sps_nal = nal_unit.to_vec();
      }
      NALType::PPS => self.pps_nal = nal_unit.to_vec(),
      NALType::AUD => {}
      _ => {
        self.media_nal.push(NalRep{
          nal_unit: nal_unit.to_vec(),
          pts: self.current_pts,
          dts: self.current_dts,
        })
      }
    }
  }

  fn flush_final_media(&mut self) {
    self.media_nal.push(NalRep{
          nal_unit: self.bucket.to_vec(),
          pts: self.current_pts,
          dts: self.current_dts,
        });
    if let Some(cb) = &self.media_callback {
      cb(&self.media_nal);
    }
  }
}