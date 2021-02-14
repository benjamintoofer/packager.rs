use std::fmt;

pub mod error_code;

use error_code::{
  MinorError,
  MajorCode,
};

pub struct CustomError {
  pub major: MajorCode,
  pub minor: u8,
  pub message: String,
  pub debug_message: String,
  pub file: &'static str,
  pub line: u32
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.message)
  }
}

impl fmt::Debug for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{{ messsage: {}, file: {}, line: {} }}", self.debug_message, self.file, self.line)
  }
}

pub fn construct_error(major: MajorCode, minor: Box<dyn MinorError>, debug_message: String, file: &'static str, line: u32) -> CustomError {
  CustomError{
    major,
    minor: minor.code(),
    message: minor.message(),
    debug_message,
    file,
    line
  }
}