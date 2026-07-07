impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn ok(self) -> Result<T, NoneError> {
        self.ok_or_else(|| NoneError(std::panic::Location::caller()))
    }
}
