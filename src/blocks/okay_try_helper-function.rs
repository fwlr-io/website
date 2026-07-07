fn strip_prefix(string: &str, prefix: &str) -> io::Result<String> {
    match string.strip_prefix(prefix) {
        Some(s) => Ok(s.to_string()),
        None => Err(io::Error::other("no prefix")),
    }
}
