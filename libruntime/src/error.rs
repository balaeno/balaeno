use std::error::Error;
use std::fmt::{self, Display, Formatter};


#[derive(strum_macros::Display, Debug)]
pub enum ErrorType {
    Runtime,
    Container,
}
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub error_type: ErrorType,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, error type: {}", self.message, self.error_type)
    }
}

impl Error for RuntimeError {}
