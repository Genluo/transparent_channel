use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ContentTypeError {
}

impl fmt::Display for ContentTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid content type")
    }
}

impl error::Error for ContentTypeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl ContentTypeError {
    pub fn new() -> ContentTypeError {
        ContentTypeError {}
    }
}
