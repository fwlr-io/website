fn read_file(file: &Path) -> Option<String> {
    match fs::read_to_string(file) {
        Ok(x) => Some(x),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}
