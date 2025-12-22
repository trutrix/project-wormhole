

/// Standardizes a given ESM path  
/// Replaces backslashes with forward slashes,  
/// removes null characters,  
/// converts to lowercase,  
/// and removes leading `./` if present.
/// TODO: Verify behavior with more examples.
pub fn normalize_esm_path(path: &str) -> String {
    let mut path = path
        .replace('\\', "/")
        .replace('\0', "")
        .to_lowercase();

    if path.starts_with("./") {
        path = path[2..].to_string();
    }
    
    path
}