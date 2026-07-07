fn read_file(file: &Path) -> Option<String> {
    let read = fs::read_to_string(file);
    if let Ok(string) = read {
        return Some(string);
    };
    if let Err(error) = read {
        eprintln!("{}", error);
    };
    return None;
}
