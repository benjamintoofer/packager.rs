use std::process::Command;
use std::thread;

use super::{VideoResolution, AudioSampleRates};
use regex::Regex;
pub struct FFMPEG {

}

impl FFMPEG {
  pub fn determine_sizes_to_transcode() {

  }
  pub fn transcode(input:&str, output: &str, sizes: Vec<VideoResolution>) {
    
    let mut ffmpeg_command = Command::new("ffmpeg");
    let mut args: Vec<String> = vec![
      "-i".to_string(),
      input.to_string()
    ];
    
    for size in sizes {
      args.append(&mut FFMPEG::args(size, output));
    }

    let handle  = thread::spawn(move || {
      let mut child_process = ffmpeg_command
        .args(args)
        .spawn()
        .expect("Failed to execute command");
      
      child_process.wait().unwrap();
    });

    handle.join().unwrap();
  }

  pub fn test_method_1(input:&str) {
    let mut ffmpeg_command = Command::new("ffmpeg");
    let mut args: Vec<String> = vec![
      "-i".to_string(),
      input.to_string()
    ];

    /**
    ffmpeg -y -i input.mkv \
-filter_complex "[0:v]format=yuv420p,yadif,split=3[in1][in2][in3];[in1]scale=1920:1080[hd];[in2]scale=720:576,hflip[sd];[in3]fps=1/5,scale=320:180[thumbnails];[0:a]aresample=48000,asplit=2[a1][a2]" \
-map [hd] -map [a1] hd.mov \
-map [sd] -map [a2] sd-flipped.mp4 \
    */
    
    // for size in sizes {
    //   args.append(&mut FFMPEG::args(size, output));
    // }
    args.append(&mut FFMPEG::test_args());
    let handle  = thread::spawn(move || {
      let mut child_process = ffmpeg_command
        .args(args)
        .spawn()
        .expect("Failed to execute command");
      
      child_process.wait().unwrap();
    });

    handle.join().unwrap();
  }

  fn generate_filter_complex(video_resolutions: Vec<VideoResolution>, audio_rates: Vec<AudioSampleRates>) -> String {
    let regex: Regex = Regex::new(r";\\\n$").unwrap();
    let mut filter_complex_str = format!("\"[0:v]format=yuv420p,yadif,split={}",video_resolutions.len());

    // Create the split input for video bitrates; Ex: [vin0][vin1][vin2];
    for (i, _) in video_resolutions.iter().enumerate() {
      filter_complex_str.push_str(format!("[vin{}]",i).as_ref());
    }
    filter_complex_str.push_str(";\\\n");
    // Create the fps and scale filter for each video bitrate; Ex: [vin0]fps=60000/1001,scale=1280:720[vout0];\
    for (i, vid_res) in video_resolutions.iter().enumerate() {
      let temp_str = format!("[vin{}]fps={},scale={}[vout{}];\\\n",i, vid_res.get_fps_str(), vid_res.get_scale(),i);
      filter_complex_str.push_str(temp_str.as_ref());
    }

    if audio_rates.len() != 0 {
      filter_complex_str.push_str(format!("[0:a]asplit={}",audio_rates.len()).as_ref());
      for (i, _) in audio_rates.iter().enumerate() {
        filter_complex_str.push_str(format!("[ain{}]",i).as_ref());
      }
      filter_complex_str.push_str(";\\\n");
      // Create the fps and scale filter for each audio sample rate; Ex: [a0]aresample=48000[aout0];\
      for (i, aud_rate) in audio_rates.iter().enumerate() {
        let temp_str = format!("[ain{}]aresample={}[aout{}];\\\n",i, aud_rate.value(),i);
        filter_complex_str.push_str(temp_str.as_ref());
      }
    }
    regex.replace_all(filter_complex_str.as_ref(), "\" \\\n").to_string()
  }

  /**
-map "[hd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/1080_60.mp4 \
-map "[hd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/1080_30.mp4 \
-map "[sd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/720_60.mp4 \
-map "[sd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/720_30.mp4 \
-map "[lowsd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/480_30.mp4 \
-map "[a10]" -map_metadata:s:a 0:s:a ~/Desktop/48000_a.mp4 \
-map "[a20]" -map_metadata:s:a 0:s:a ~/Desktop/96000_a.mp4
  */
  fn generate_mappings(video_resolutions: Vec<VideoResolution>, audio_rates: Vec<AudioSampleRates>) -> String {
    let mut mapping_str = String::new();
    for (i, vid_res) in video_resolutions.iter().enumerate() {
      mapping_str.push_str(format!("-map \"[vout{}]\" -c:v libx264 -x264opts keyint={}:no-scenecut -r {} -map_metadata:s:v 0:s:v \\\n",i, (vid_res.get_fps() * 2), vid_res.get_fps()).as_ref());
    }

    if audio_rates.len() != 0 {
      for (i, _) in audio_rates.iter().enumerate() {
        mapping_str.push_str(format!("-map \"[aout{}]\" -map_metadata:s:a 0:s:a \\\n",i).as_ref());
      }
    }
    mapping_str
  }

  fn test_args() -> Vec<String> {
    return vec!["-y -i ./temp/BigBuckBunny.mp4
  -filter_complex \"[0:v]format=yuv420p,yadif,split=3[in1][in2][in3];[in1]scale=1920:1080[hd];[in2]scale=720:576,hflip[sd];[in3]fps=1/5,scale=320:180[thumbnails];[0:a]aresample=48000,asplit=2[a1][a2]\"
  -map [hd] -map [a1] hd.mov
  -map [sd] -map [a2] sd-flipped.mp4".to_string()];
  }

  fn args(size: VideoResolution, output: &str) -> Vec<String> {
    println!("{}_{}.mp4", output, size.value());
    vec![
      "-y".to_string(), // Overrite existing files
      "-c:v".to_string(), "libx264".to_string(), // Encode using libx264 encoder
      "-x264opts".to_string(), "keyint=60:no-scenecut".to_string(), // x264 options, key interval every 60 frames
      "-s".to_string(), size.value(),
      "-r".to_string(), "30".to_string(),
      "-preset".to_string(), "veryfast".to_string(),
      "-c:a".to_string(), "aac".to_string(),
      "-sws_flags".to_string(), "bilinear".to_string(),
      format!("{}/{}.mp4", output, size.value())
      // format!("./temp/output_{}.mp4",size.value())
    ]
  }
}

// [0:a]aresample=48000,asplit=3[a1][a2]"

/**
ffmpeg -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=5[in1][in2][in3][in4][in5];\
[in1]fps=60000/1001[hd60];\
[in2]fps=30000/1001[hd30];\
[in3]fps=60000/1001,scale=1280:720[sd60];\
[in4]fps=30000/1001,scale=1280:720[sd30];\
[in5]fps=30000/1001,scale=852:480[lowsd30];\
[0:a]aresample=48000,asplit=2[a1]" \
-map "[hd60]" -map_metadata:s:v 0:s:v ~/Desktop/1080_60.mp4 \
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
-map "[hd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/1080_60.mp4 \
-map "[hd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/1080_30.mp4 \
-map "[sd60]" -c:v libx264 -x264opts keyint=120:no-scenecut -r 60 -map_metadata:s:v 0:s:v ~/Desktop/720_60.mp4 \
-map "[sd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/720_30.mp4 \
-map "[lowsd30]" -c:v libx264 -x264opts keyint=60:no-scenecut -r 30 -map_metadata:s:v 0:s:v ~/Desktop/480_30.mp4 \
-map "[a10]" -map_metadata:s:a 0:s:a ~/Desktop/48000_a.mp4 \
-map "[a20]" -map_metadata:s:a 0:s:a ~/Desktop/96000_a.mp4


ffmpeg -y -i /Users/benjamintoofer/Developer/Packager/temp/ToS-4k_30sec.mp4 \
-filter_complex "[0:v]format=yuv420p,yadif,split=1[in1];\
[in1]fps=30000/1001,scale=852:480[lowsd30];\
[0:a]aresample=48000,asplit=1[a1]" \
-map "[lowsd30]" -map_metadata 0:s:0 ~/Desktop/480_30.mp4 \
-map "[a1]" -map_metadata 0:s:0 ~/Desktop/48000_a.mp4
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
    let output = FFMPEG::generate_filter_complex(vid_res, aud_rates);

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
    let output = FFMPEG::generate_filter_complex(vid_res, aud_rates);

    assert_eq!(output, expected_output);
  }

  #[test]
  fn test_generate_mappings_without_audio() {
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![];
    let output = FFMPEG::generate_mappings(vid_res, aud_rates);

    assert_eq!(output, "");
  }

  #[test]
  fn test_generate_mappings_with_audio() {
    let vid_res = vec![
      VideoResolution::_1080_60,
      VideoResolution::_720_30
    ];
    let aud_rates = vec![
      AudioSampleRates::_96k,
    ];
    let output = FFMPEG::generate_mappings(vid_res, aud_rates);

    assert_eq!(output, "");
  }
}