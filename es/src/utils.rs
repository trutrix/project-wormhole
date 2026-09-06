use std::{io::Error, path::{Path, PathBuf}};

use crate::consts::*;

/// Returns paths to every file in a directory that ends with .esl, .esp, and .esm
pub fn get_es_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Error> {

    let result = std::fs::read_dir(path)?;
    let mut paths = Vec::new();
    
    for entry in result {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                let lc = ext.to_ascii_lowercase();
                let fe = lc.to_str().expect("File names in directory contain non-ascii extensions.");

                if TARGET_EXTS.contains(&fe) {
                    paths.push(entry.path());
                }
            }
        }
    }

    Ok(paths)
}