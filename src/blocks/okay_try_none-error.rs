#[derive(Debug)]
pub struct NoneError(&'static std::panic::Location<'static>);
impl std::error::Error for NoneError {}
impl std::fmt::Display for NoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "None is not `.ok` at {}", self.0)
    }
}

impl From<NoneError> for std::io::Error {
    fn from(none_error: NoneError) -> std::io::Error {
        std::io::Error::other(format!("{none_error}"))
    }
}
