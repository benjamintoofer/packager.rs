use std::{fs, process::Command};
use std::thread;

use super::{VideoResolution, AudioSampleRates};
use regex::Regex;
pub struct FFMPEG {}

impl FFMPEG {
  pub fn determine_sizes_to_transcode() {

  }
  pub fn transcode(input:&str, output: &str, video_res: &Vec<VideoResolution>, audio_rates: &Vec<AudioSampleRates>) {
    
    // Create directories for video and audio
    video_res
      .iter()
      .for_each(|res| fs::create_dir_all(format!("{}/{}/{}_{}",output, "video", res.value(), res.get_fps())).unwrap());

    audio_rates
      .iter()
      .for_each(|res|fs::create_dir_all(format!("{}/{}/{}",output, "audio", res.value())).unwrap());

    let mut ffmpeg_command = Command::new("ffmpeg");
    let mut args: Vec<String> = vec![
      "-y".to_string(),
      "-i".to_string(),
      input.to_string()
    ];
    let mut filter_arg = vec![
      "-filter_complex".to_string(),
      FFMPEG::generate_filter_complex(&video_res, &audio_rates),
    ];
    args.append(&mut filter_arg);
    let mut mappings = FFMPEG::generate_mappings(&video_res, &audio_rates, output);
    args.append(&mut mappings);

    let handle  = thread::spawn(move || {
      let mut child_process = ffmpeg_command
        .args(args)
        .spawn()
        .expect("Failed to execute command");
      
      child_process.wait().unwrap();
    });
    handle.join().unwrap();
  }


  fn generate_filter_complex(video_resolutions: &Vec<VideoResolution>, audio_rates: &Vec<AudioSampleRates>) -> String {
    let regex: Regex = Regex::new(r";$").unwrap();
    let mut filter_complex_str = format!("[0:v]format=yuv420p,yadif,split={}",video_resolutions.len());

    // Create the split input for video bitrates; Ex: [vin0][vin1][vin2];
    for (i, _) in video_resolutions.iter().enumerate() {
      filter_complex_str.push_str(format!("[vin{}]",i).as_ref());
    }
    filter_complex_str.push_str(";");
    // Create the fps and scale filter for each video bitrate; Ex: [vin0]fps=60000/1001,scale=1280:720[vout0];\
    for (i, vid_res) in video_resolutions.iter().enumerate() {
      let temp_str = format!("[vin{}]fps={},scale={}[vout{}];",i, vid_res.get_fps_str(), vid_res.get_scale(),i);
      filter_complex_str.push_str(temp_str.as_ref());
    }

    if audio_rates.len() != 0 {
      filter_complex_str.push_str(format!("[0:a]asplit={}",audio_rates.len()).as_ref());
      for (i, _) in audio_rates.iter().enumerate() {
        filter_complex_str.push_str(format!("[ain{}]",i).as_ref());
      }
      filter_complex_str.push_str(";");
      // Create the fps and scale filter for each audio sample rate; Ex: [a0]aresample=48000[aout0];\
      for (i, aud_rate) in audio_rates.iter().enumerate() {
        let temp_str = format!("[ain{}]aresample={}[aout{}];",i, aud_rate.value(),i);
        filter_complex_str.push_str(temp_str.as_ref());
      }
    }
    regex.replace(filter_complex_str.as_ref(), "").to_string()
  }

  fn generate_mappings(video_resolutions: &Vec<VideoResolution>, audio_rates: &Vec<AudioSampleRates>, output: &str) -> Vec<String> {
    let mut str_vec: Vec<String> = vec![];
    let mut temp_vec: Vec<String>;
    for (i, vid_res) in video_resolutions.iter().enumerate() {
      temp_vec = vec![
        "-map".to_string(),format!("[vout{}]",i),
        "-c:v".to_string(), "libx264".to_string(),
        "-x264opts".to_string(), format!("keyint={}:no-scenecut",(vid_res.get_fps() * 2)),
        "-r".to_string(), format!("{}", vid_res.get_fps()),
        "-map_metadata:s:v".to_string(), "0:s:v".to_string(),
        format!("./{}/video/{}_{}/media.mp4",output,vid_res.value(),vid_res.get_fps())
      ];
      str_vec.append(&mut temp_vec);
    }
    
    if audio_rates.len() != 0 {
      for (i, aud) in audio_rates.iter().enumerate() {
        temp_vec = vec![
          "-map".to_string(),format!("[aout{}]",i),
          "-map_metadata:s:a".to_string(), "0:s:a".to_string(),
          format!("./{}/audio/{}/media.mp4",output, aud.value())
        ];
        str_vec.append(&mut temp_vec);
      }
    }

    str_vec
  }
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
  fn test_generate_filter_complex_without_audio() {
    let expected_output = "[0:v]format=yuv420p,yadif,split=2[vin0][vin1];[vin0]fps=60000/1001,scale=1920:1080[vout0];[vin1]fps=30000/1001,scale=1280:720[vout1]";
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![];
    let output = FFMPEG::generate_filter_complex(&vid_res, &aud_rates);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn test_generate_filter_complex_with_audio() {
    let expected_output = "[0:v]format=yuv420p,yadif,split=2[vin0][vin1];[vin0]fps=60000/1001,scale=1920:1080[vout0];[vin1]fps=30000/1001,scale=1280:720[vout1];[0:a]asplit=1[ain0];[ain0]aresample=96000[aout0]";
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![
      AudioSampleRates::_96k,
    ];
    let output = FFMPEG::generate_filter_complex(&vid_res, &aud_rates);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn test_generate_mappings_without_audio() {
    let expected_output:Vec<&str> = vec!["-map", "[vout0]", "-c:v", "libx264", "-x264opts", "keyint=120:no-scenecut", "-r", "60", "-map_metadata:s:v", "0:s:v", "./output/video/1920x1080_60.mp4", "-map", "[vout1]", "-c:v", "libx264", "-x264opts", "keyint=60:no-scenecut", "-r", "30", "-map_metadata:s:v", "0:s:v", "./output/video/1280x720_30.mp4"];
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![];
    let output = FFMPEG::generate_mappings(&vid_res, &aud_rates, "output");
    output.to_vec();
    assert_eq!(output.to_vec(), expected_output);
  }

  #[test]
  fn test_generate_mappings_with_audio() {
    let expected_output: Vec<&str> = vec!["-map", "[vout0]", "-c:v", "libx264", "-x264opts", "keyint=120:no-scenecut", "-r", "60", "-map_metadata:s:v", "0:s:v", "./output/video/1920x1080_60.mp4", "-map", "[vout1]", "-c:v", "libx264", "-x264opts", "keyint=60:no-scenecut", "-r", "30", "-map_metadata:s:v", "0:s:v", "./output/video/1280x720_30.mp4", "-map", "[aout0]", "-map_metadata:s:a", "0:s:a", "./output/audio/96000.mp4"];
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![
      AudioSampleRates::_96k,
    ];
    let output = FFMPEG::generate_mappings(&vid_res, &aud_rates, "output");

    assert_eq!(output.to_vec(), expected_output);
  }
}