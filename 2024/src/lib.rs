pub fn load_file<P: Into<String>>(file_path: P) -> String {
    return std::fs::read_to_string(file_path.into()).unwrap();
}
