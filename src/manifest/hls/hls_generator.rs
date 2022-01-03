use std::collections::HashMap;

use crate::media::{TrackInfo, MediaInfo, TrackType};
use crate::container::isobmff::{map_iso_639_2_to_639_1, map_iso_639_2_to_name};
use crate::manifest::hls::hls_writer::HLSWriter;
use crate::manifest::hls::HLSMediaType;

use super::HLSVersion;

const DEFAULT_VERSION: HLSVersion = HLSVersion::_7;

pub struct HLSGenerator {

}

impl HLSGenerator {
  // Need to know:
  // - which veriosn of HLS
  // - All avaialable tracks (audio+langiage, video+resolution/bitrate)

  // 1. Bucket tracks into their own group ids with its own track tracktype. 
  //  1a. NEED TO TEST DIFFERENT COMBOS (mutliple audio tracks/1 group; mutliple audio tracks/each group id; Combos for CC and Subtitles too) 
  // 2. Identify video tracks with groups of AUDIO, SUBTITLES, CC
  //  2a. Combine (codecs) and (bitrates) for audio and video
  // 3. Assign groupd ids of AUDIO, CC, SUBTITLES to the video stream inf
  // BONUS: Implement IFrame playlist generation
  pub fn generate_master(metadata: &MediaInfo) -> String {
    let mut audio_groups = HashMap::new();
    let audio_tracks: Vec<&TrackInfo> = metadata.track_infos
      .iter()
      .filter(|ti|ti.track_type == TrackType::AUDIO)
      .collect();

    let video_tracks: Vec<&TrackInfo> = metadata.track_infos
      .iter()
      .filter(|ti|ti.track_type == TrackType::VIDEO)
      .collect();

    let is_independent_segments = metadata.track_infos
      .iter()
      .filter(|ti|ti.track_type == TrackType::VIDEO)
      .all(|ti|ti.segments_start_with_i_frame == true);
      
    // GENERATE GROUPS HERE
    let mut hls_writer = HLSWriter::create_writer();
    let writer = hls_writer.start_hls()
    .version(DEFAULT_VERSION);

    // Addd independent segments tag if we detect that all tracks have independent segments
    if is_independent_segments {
      writer.independent();
    }
    
    writer.new_line()
    .comment("This manifest is created by Luma")
    .new_line();

    audio_tracks.iter().for_each(|track|{
      writer.media(
        HLSMediaType::AUDIO, 
        track.audio_group_id.unwrap_or_default(), 
        &map_iso_639_2_to_name( &track.language), 
        Some(&track.path), 
        Some(&map_iso_639_2_to_639_1(&track.language)), 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        Some(track.audio_channels));
    });

    writer.new_line();

    audio_tracks.iter().for_each(|a_track| {
      if !audio_groups.contains_key(a_track.audio_group_id.unwrap_or_default()) {
        video_tracks.iter().for_each(|track|{
          writer.stream_inf(
            &track.path.replace("./output", "").replace("media_frag.mp4", "playlist.m3u8"), 
            track.max_bandwidth, 
            Some(track.average_bandwidth), 
            Some(track.frame_rate), 
            None, 
            None, 
            Some(&format!("{}x{}",track.width, track.height)), 
            None, 
            Some(&format!("{},{}",&track.codec, a_track.codec)),
            a_track.audio_group_id,
            None, 
            None,
            None
          );
        });

        // Add audio group to seen audio groups map
        audio_groups.insert(a_track.audio_group_id.unwrap_or_default(), true);
      }
      writer.new_line();
    });

    writer.finish().to_string()
  }

  // Need to know:
  // - which version of HLS
  // - if each segment starts with an Iframe (independent segments tag)
  // - if byterange
  // - maximum segment duration
  // - segment  info (path, duration)
  // TODO (benjamintoofer@gmail.com): Come back to this and finish
  pub fn generate_media_playlist<'a>(metadata: &'a TrackInfo) -> String {

    let mut hls_writer = HLSWriter::create_writer();

    let writer  = hls_writer.start_hls()
      .new_line()
      .comment("This manifest is created by Luma")
      .new_line()
      .target_duration(metadata.maximum_segment_duration as u8)
      .version(DEFAULT_VERSION)
      .map(&metadata.path, Option::Some(metadata.init_segment.bytes), Option::Some(metadata.init_segment.offset))
      .new_line();

    for media_seg in &metadata.segments {
      writer.inf(media_seg.duration, Option::None);
      writer.byte_range(media_seg.bytes, media_seg.offset, &metadata.path);
    }

    writer.endlist().finish().to_string()
  }

  pub fn generate_i_frame_playlist() {

  }
}