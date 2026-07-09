pub trait OptionExt<T> {
    fn ok(self) -> anyhow::Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn ok(self) -> anyhow::Result<T> {
        self.ok_or(anyhow::anyhow!(
            "None is not `ok` at {}",
            std::panic::Location::caller()
        ))
    }
}
