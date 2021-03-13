use std::process::Command;
use std::thread;

use super::VideoResolution;

pub struct FFMPEG {

}

impl FFMPEG {
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