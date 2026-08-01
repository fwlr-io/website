use std::fmt;
use std::panic::Location;

pub struct NoneError(&'static Location<'static>);

impl fmt::Debug for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Called `ok()?` on a `None` at {}", self.0)
    }
}
impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Called `ok()?` on a `None` at {}", self.0)
    }
}

impl std::error::Error for NoneError {}
