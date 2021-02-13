use std::fmt;

pub mod isobmff_errors;

#[derive(Clone)]
pub struct ParseUnsignedError {
  pub message: String,
  pub file: &'static str,
  pub line: u32
}

impl fmt::Display for ParseUnsignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred attempting to parse and unsigned value")
    }
}

impl fmt::Debug for ParseUnsignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ messsage: {}, file: {}, line: {} }}", self.message, self.file, self.line)
    }
}