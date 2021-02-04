use env_logger::{Builder};

pub struct Logger {
  // instance: Box<Self>
}

impl Logger {

  pub fn get_logger(name: &'static str) -> Logger {
    Logger{}
  }
}