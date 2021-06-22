use std::process::Command;
use std::thread;

use super::{VideoResolution, AudioSampleRates};
use regex::Regex;
pub struct FFMPEG {}

impl FFMPEG {
  pub fn determine_sizes_to_transcode() {

  }
  pub fn transcode(input:&str, output: &str, video_res: Vec<VideoResolution>, audio_rates: Vec<AudioSampleRates>) {
    
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
    let mut mappings = FFMPEG::generate_mappings(&video_res, &audio_rates);
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

  fn generate_mappings(video_resolutions: &Vec<VideoResolution>, audio_rates: &Vec<AudioSampleRates>) -> Vec<String> {
    let mut str_vec: Vec<String> = vec![];
    let mut temp_vec: Vec<String>;
    for (i, vid_res) in video_resolutions.iter().enumerate() {
      temp_vec = vec![
        "-map".to_string(),format!("[vout{}]",i),
        "-c:v".to_string(), "libx264".to_string(),
        "-x264opts".to_string(), format!("keyint={}",(vid_res.get_fps() * 2)),
        "-r".to_string(), format!("{}", vid_res.get_fps()),
        "-map_metadata:s:v".to_string(), "0:s:v".to_string(),
        format!("./output/test/{}_{}.mp4",vid_res.value(),vid_res.get_fps())
      ];
      str_vec.append(&mut temp_vec);
    }
    
    if audio_rates.len() != 0 {
      for (i, aud) in audio_rates.iter().enumerate() {
        temp_vec = vec![
          "-map".to_string(),format!("[aout{}]",i),
          "-map_metadata:s:a".to_string(), "0:s:a".to_string(),
          format!("./output/test/{}.mp4",aud.value())
        ];
        str_vec.append(&mut temp_vec);
      }
    }

    str_vec
  }
}

/**
ffmpeg -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=5[in1][in2][in3][in4][in5];\
[in1]fps=60000/1001[hd60];\
[in2]fps=30000/1001[hd30];\
[in3]fps=60000/1001,scale=1280:720[sd60];\
[in4]fps=30000/1001,scale=1280:720[sd30];\
[in5]fps=30000/1001,scale=852:480[lowsd30];\
[0:a]aresample=48000,asplit=2[a1]" \
-map "[hd60]" -map_metadata:s:v 0:s:v -movflags frag_keyframe+empty_moov ~/Desktop/1080_60.mp4 \
-map "[hd30]" -map_metadata:s:v 0:s:v ~/Desktop/1080_30.mp4 \
-map "[sd60]" -map_metadata:s:v 0:s:v ~/Desktop/720_60.mp4 \
-map "[sd30]" -map_metadata:s:v 0:s:v ~/Desktop/720_30.mp4 \
-map "[lowsd30]" -map_metadata:s:v 0:s:v ~/Desktop/480_30.mp4 \
-map "[a1]" -map_metadata:s:a 0:s:a ~/Desktop/48000_a.mp4

ffmpeg -benchmark -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=5[in1][in2][in3][in4][in5];\
[in1]fps=60000/1001[hd60];\
[in2]fps=30000/1001[hd30];\
[in3]fps=60000/1001,scale=1280:720[sd60];\
[in4]fps=30000/1001,scale=1280:720[sd30];\
[in5]fps=30000/1001,scale=852:480[lowsd30];\
[0:a]asplit=2[a1][a2];\
[a1]aresample=48000[a10];\
[a2]aresample=96000[a20]" \
-map "[hd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v -movflags frag_keyframe+empty_moov ~/Desktop/1080_60.mp4 \
-map "[hd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v -movflags frag_keyframe+empty_moov ~/Desktop/1080_30.mp4 \
-map "[sd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v -movflags frag_keyframe+empty_moov ~/Desktop/720_60.mp4 \
-map "[sd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v -movflags frag_keyframe+empty_moov ~/Desktop/720_30.mp4 \
-map "[lowsd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v -movflags frag_keyframe+empty_moov ~/Desktop/480_30.mp4 \
-map "[a10]" -map_metadata:s:a 0:s:a ~/Desktop/48000_a.mp4 \
-map "[a20]" -map_metadata:s:a 0:s:a ~/Desktop/96000_a.mp4


ffmpeg -benchmark -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=5[in1][in2][in3][in4][in5];\
[in1]fps=60000/1001[hd60];\
[in2]fps=30000/1001[hd30];\
[in3]fps=60000/1001,scale=1280:720[sd60];\
[in4]fps=30000/1001,scale=1280:720[sd30];\
[in5]fps=30000/1001,scale=852:480[lowsd30];\
[0:a]asplit=2[a1][a2];\
[a1]aresample=48000[a10];\
[a2]aresample=96000[a20]" \
-map "[hd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/1080_60.mp4 \
-map "[hd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/1080_30.mp4 \
-map "[sd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/720_60.mp4 \
-map "[sd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/720_30.mp4 \
-map "[lowsd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/480_30.mp4 \
-map "[a10]" -map_metadata:s:a 0:s:a ~/Desktop/48000_a.mp4 \
-map "[a20]" -map_metadata:s:a 0:s:a ~/Desktop/96000_a.mp4
*/

/**
ffmpeg -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-c:v libx264 -x264opts keyint=120:no-scenecut -s 1920x1080 -r 60 -preset "veryfast" -c:a aac -sws_flags bilinear ~/Desktop/test_ffmpeg/twitch/twitch_test_1080_60.mp4 \
-c:v libx264 -x264opts keyint=60:no-scenecut -s 1920x1080 -r 30 -preset "veryfast" -c:a aac -sws_flags bilinear ~/Desktop/test_ffmpeg/twitch/twitch_test_1080_30.mp4 \
-c:v libx264 -x264opts keyint=120:no-scenecut -s 1280x720 -r 60 -preset "veryfast" -c:a aac -sws_flags bilinear ~/Desktop/test_ffmpeg/twitch/twitch_test_720_60.mp4 \
-c:v libx264 -x264opts keyint=60:no-scenecut -s 1280x720 -r 30 -preset "veryfast" -c:a aac -sws_flags bilinear ~/Desktop/test_ffmpeg/twitch/twitch_test_720_30.mp4 \
-c:v libx264 -x264opts keyint=60:no-scenecut -s 852x480 -r 30 -preset "veryfast" -c:a aac -sws_flags bilinear ~/Desktop/test_ffmpeg/twitch/twitch_test_480.mp4

Hey I’m new to ffmpeg but I was hoping to learn and understand how it works. I’m currently messing around with this command
ffmpeg -y -i input.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=2[in1][in2];\
[in1]fps=60000/1001[hd60];\
[in2]fps=30000/1001[hd30];\
-map "[hd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/1080_60.mp4 \
-map "[hd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/1080_30.mp4 \

ffmpeg -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=5[in1][in2][in3][in4][in5];\
[in1]fps=60000/1001[hd60];\
[in2]fps=30000/1001[hd30];\
-map "[hd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/1080_60.mp4 \
-map "[hd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/1080_30.mp4 \
*/

// ffmpeg -ss 00:07:45.0 -i ~/Developer/Packager/temp/ToS-4k.mp4 -c copy -t 00:00:30.0 ~/Developer/Packager/temp/ToS-4k_30sec.mp4
#[cfg(test)]
mod tests {
  use super::*;

    #[test]
  fn test_generate_filter_complex_without_audio() {
    let expected_output = "\"[0:v]format=yuv420p,yadif,split=2[vin0][vin1];\\
[vin0]fps=60000/1001,scale=1920:1080[vout0];\\
[vin1]fps=30000/1001,scale=1280:720[vout1]\" \\\n";
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
    let expected_output = "\"[0:v]format=yuv420p,yadif,split=2[vin0][vin1];\\
[vin0]fps=60000/1001,scale=1920:1080[vout0];\\
[vin1]fps=30000/1001,scale=1280:720[vout1];\\
[0:a]asplit=1[ain0];\\
[ain0]aresample=96000[aout0]\" \\\n";
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
    let expected_output:Vec<String> = vec!["-map \"[vout0]\" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v \\\n-map \"[vout1]\" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v".to_string()];
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![];
    let output = FFMPEG::generate_mappings(&vid_res, &aud_rates);
    output.to_vec();
    assert_eq!(output.to_vec(), expected_output);
  }

  #[test]
  fn test_generate_mappings_with_audio() {
    let expected_output = vec!["-map \"[vout0]\" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v \\\n-map \"[vout1]\" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v \\\n-map \"[aout0]\" -map_metadata:s:a 0:s:a".to_string()];
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![
      AudioSampleRates::_96k,
    ];
    let output = FFMPEG::generate_mappings(&vid_res, &aud_rates);

    assert_eq!(output.to_vec(), expected_output);
  }
}