pub trait OptionExt<T> {
    fn ok(self) -> Result<T, NoneError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok(self) -> Result<T, NoneError> {
        self.ok_or_else(NoneError::default)
    }
}
