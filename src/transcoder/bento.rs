use std::process::Command;
use std::thread;

pub struct Bento;

impl Bento {
  pub fn fragment(paths: Vec<String>) {
    let mut children = vec![];
    for path in paths {
      // let mut bento_command = Command::new("mp4fragment");

      // Video
      let args = Bento::args(&path, "video");
      children.push(thread::spawn(|| {
          Command::new("mp4fragment")
            .args(args)
            .spawn()
            .expect("Failed to execute command");
      }));

      // Audio
      let args = Bento::args(&path, "audio");
      children.push(thread::spawn(move || {
          Command::new("mp4fragment")
            .args(args)
            .spawn()
            .expect("Failed to execute command");
      }));
    }
    
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
  }

  fn args(path: &String, track: &str) -> Vec<String> {
    println!("PATH: {}", path);
    vec![
      "--index".to_string(),
      "--fragment-duration".to_string(), "2000".to_string(),
      "--timescale".to_string(), "90000".to_string(),
      "--track".to_string(), track.to_string(),
      path.to_string(),
      path.replace(".mp4", format!("_frag_{}.mp4",track).as_str())
    ]
  }
}