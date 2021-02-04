use std::fmt;

#[derive(Debug, Clone)]
struct ISOBMFFError;

impl fmt::Display for ISOBMFFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO")
    }
}