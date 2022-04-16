use std::{fs::File, io::Write};
use crate::container::transport_stream::{
    pes_packet, program_association_table::ProgramAssociationTable,
    program_map_table::ProgramMapTable, ts_packet,
};
use crate::container::writer::mp4_writer::Mp4Writer;
use crate::error::CustomError;
use crate::container::isobmff::HandlerType;
use crate::container::remux::extractor::{TSExtractor,get_ts_extractor};
use crate::error::{construct_error, error_code::{RemuxMinorCode, MajorCode}};
use crate::container::transport_stream::elementary_stream_type::ElementaryStreamType;


pub mod extractor;

static SYNC_BYTE: u8 = 0x47;
static TS_PACKET_SIZE: usize = 188;

pub fn remux_ts_to_mp4(ts_file: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CustomError> {
  let mut video_ts_extractor: Option<Box<dyn TSExtractor>> = None;
  let mut audio_ts_extractor: Option<Box<dyn TSExtractor>> = None;
  let mut index = 0usize;

  let mut pat: ProgramAssociationTable;
  let mut pmt: ProgramMapTable;

  let mut program_map_pid: u16 = u16::max_value();
  let mut video_elem_pid = u16::max_value();
  let mut audio_elem_pid = u16::max_value();

  while index < ts_file.len() {
    if ts_file[index] != SYNC_BYTE {
      // Out of sync so find the next sync point
      index = index + 1;
      continue;
    }
    let packet = ts_packet::TransportPacket::parse(ts_file[index..(index + TS_PACKET_SIZE)].as_ref())?;
    // ProgramAssociationTable
    if packet.pid == 0 {
      pat = ProgramAssociationTable::parse(packet.data, packet.payload_unit_start_indicator)
        .unwrap();
      program_map_pid = pat.program_map_pid;
    }

    // ProgramMapTable
    if packet.pid == program_map_pid {
      pmt = ProgramMapTable::parse(packet.data, packet.payload_unit_start_indicator).unwrap();
      // Video
      if let Some(stream_info) = pmt.video_stream_info {
        video_elem_pid = stream_info.pid;
         if video_ts_extractor.is_none() {
          let video_extractor = init_video_extractor(stream_info.stream_type)?;
          video_ts_extractor = Some(video_extractor);
        }
      }
      // Audio
      if let Some(stream_info) = pmt.audio_stream_info {
        audio_elem_pid = stream_info.pid;
        if audio_ts_extractor.is_none() {
          let audio_extractor = init_audio_extractor(stream_info.stream_type)?;
          audio_ts_extractor = Some(audio_extractor);
        }
      }
    }

    // Video PES
    if packet.pid == video_elem_pid {
      let pes = pes_packet::PESPacket::parse(packet.data)?;
      video_ts_extractor
        .as_mut()
        .and_then(|tse| {
          tse.accumulate_pes_payload(pes).ok()
        });
    }

    // Audio PES
    if packet.pid == audio_elem_pid {
      let pes = pes_packet::PESPacket::parse(packet.data)?;
      audio_ts_extractor
        .as_mut()
        .and_then(|tse| {
          tse.accumulate_pes_payload(pes).ok()
        });
    }

    index = index + TS_PACKET_SIZE;
  }

  video_ts_extractor
    .as_mut()
    .and_then(|tse|{
      tse.flush_final_media().ok()
    });

  audio_ts_extractor
    .as_mut()
    .and_then(|tse|{
      tse.flush_final_media().ok()
    });

  Ok((vec![], vec![]))
}

pub fn remux_ts_to_mp4_media_only(ts_file: &[u8]) -> Result<Vec<u8>, CustomError> {
    // TODO
    Ok(vec![])
}

pub fn generate_error(message: String) -> CustomError {
  return  construct_error(
    MajorCode::REMUX, 
    Box::new(RemuxMinorCode::MISSING_BUILDER_DEPENDENCY_ERROR),
    message,
    file!(), 
    line!());
}

pub fn map_sample_frequency_index(index: u8) -> u32 {
  match index {
    0x0 => 96000,
    0x1 => 88200,
    0x2 => 64000,
    0x3 => 48000,
    0x4 => 44100,
    0x5 => 32000,
    0x6 => 24000,
    0x7 => 22050,
    0x8 => 16000,
    0x9 => 12000,
    0xA => 11025,
    0xB => 8000,
    0xC => 7350,
    _ => 0
  }
}

fn init_audio_extractor(stream_type: ElementaryStreamType) -> Result<Box<dyn TSExtractor>, CustomError> {
  let mut audio_extractor = get_ts_extractor(stream_type)?;
    audio_extractor.listen_for_init_data(|sample_entry_data|{
      let init_segment = Mp4Writer::create_mp4_writer()
        .timescale(44100)
        .handler(HandlerType::SOUN)
        .build_init_segment(sample_entry_data);
      
      match init_segment {
        Ok(x) => {
          let mut file = File::create("/Users/benjamintoofer/Desktop/my_own_audio_init.mp4").unwrap();
          match file.write_all(&x) {
              Ok(_) => {println!("FINISHED WRITING SEGMENT!!!")}
              Err(_) => {println!("FUCKED UP WRITING SEGMENT")}
          }
        }
        Err(err) => {
          println!("{:?}", err);
        }
      }
    });

    audio_extractor.listen_for_media_data(|media_data|{
      let media_segment = Mp4Writer::create_mp4_writer()
        .timescale(44100)
        .samples(media_data)
        .build_media_segment();
      
      match media_segment {
        Ok(x) => {
          let mut file = File::create("/Users/benjamintoofer/Desktop/my_own_audio_media.mp4").unwrap();
          match file.write_all(&x) {
              Ok(_) => {println!("FINISHED WRITING SEGMENT!!!")}
              Err(_) => {println!("FUCKED UP WRITING SEGMENT")}
          }
        }
        Err(err) => {
          println!("{:?}", err);
        }
      }
  });

  Ok(audio_extractor)
}


fn init_video_extractor(stream_type: ElementaryStreamType) -> Result<Box<dyn TSExtractor>, CustomError> {
  let mut vid_extractor = get_ts_extractor(stream_type)?;
  vid_extractor.listen_for_init_data(|sample_entry_data|{
    let init_segment = Mp4Writer::create_mp4_writer()
      .timescale(90000)
      .handler(HandlerType::VIDE)
      .build_init_segment(sample_entry_data);
    
    match init_segment {
      Ok(x) => {
        let mut file = File::create("/Users/benjamintoofer/Desktop/my_own_video_init.mp4").unwrap();
        match file.write_all(&x) {
            Ok(_) => {println!("FINISHED WRITING SEGMENT!!!")}
            Err(_) => {println!("FUCKED UP WRITING SEGMENT")}
        }
      }
      Err(err) => {
        println!("{:?}", err);
      }
    }
  });

  vid_extractor.listen_for_media_data(|media_data|{
      let media_segment = Mp4Writer::create_mp4_writer()
      .timescale(90000)
      .samples(media_data)
      .build_media_segment();
  });

  Ok(vid_extractor)
}
