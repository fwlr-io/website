impl<T> Option<T> {
    fn ok(self) -> Result<T, NoneError> {
        self.ok_or_else(NoneError::default)
    }
}
