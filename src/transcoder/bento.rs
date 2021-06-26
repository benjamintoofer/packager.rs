use std::{fs, process::Command};
use std::thread;

pub struct Bento;

impl Bento {
  pub fn fragment(paths: Vec<String>) {
    let mut children = vec![];
    for path in paths.clone() {
      let mut timescale = "90000".to_string();
      if path.contains("audio") {
        timescale = "48000".to_string();
      }
      // Fragment each mp4
      let args = Bento::args(&path, timescale);
      children.push(thread::spawn(|| {
          let mut child_process = Command::new("mp4fragment")
            .args(args)
            .spawn()
            .expect("Failed to execute command");
          child_process.wait().unwrap();
      }));
    }
    
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    for path in paths {
      fs::remove_file(&path).expect(format!("Failed removing {}", &path).as_ref());
    }
  }

  fn args(path: &String, timescale: String) -> Vec<String> {
    vec![
      "--index".to_string(),
      "--fragment-duration".to_string(), "2000".to_string(),
      "--timescale".to_string(), timescale,
      path.to_string(),
      path.replace(".mp4", format!("_frag.mp4").as_str())
    ]
  }
}
