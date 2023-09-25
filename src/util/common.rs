use std::path::Path;

pub fn module_name(filepath: &str) -> &str {
    let basename = file_basename(filepath);
    "123124"
}

pub fn file_basename(filename: &str) -> &str {
    let path = Path::new(filename);
    path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("<embedded>")
}
