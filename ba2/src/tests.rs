#![allow(unused_imports)]
use std::{path::PathBuf, str::FromStr};

use super::dev::*;

#[test]
#[ignore = "Long test, only run when needed"]
pub fn test_general_archive() {
    let path = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4 - Meshes.ba2";
    let mut ba2 = BA2Archive::open(path).unwrap();
    println!("{:?}", ba2);

    let all_files = ba2.read_all_files();
    println!("Files: {}", all_files.len());
    //println!("{:?}", archive.files);
    //println!("{:?}", archive.files);
}


#[test]
#[ignore = "Long test, only run when needed"]
fn test_texture_archive() {
    use std::io::Write;
    let path: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4 - Textures4.ba2";
    let mut ba2 = BA2Archive::open(path).unwrap();
    println!("{:?}", ba2);

    let img = ba2.read_file(&"Textures\\Actors\\Alien\\Alien_01_D.DDS").unwrap();
    std::fs::File::create(&"out/Alien_01_D.DDS".to_lowercase()).unwrap().write_all(&img).unwrap();

    let all_files = ba2.read_all_files();
    println!("Files: {}", all_files.len());
    //println!("{:?}", file.files);
    //println!("File length: {}", data.len());

}

#[test]
#[ignore = "Long test, only run when needed"]
fn test_archive_dir() {
    use std::io::Write;
    let path = PathBuf::from_str("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data").unwrap();
    let _ba2 = BA2ArchiveGroup::open_all(path).unwrap();
}

#[test]
fn test_header_serialization() {
    let header = crate::header::BA2Header {
        id: *b"BTDX",
        version: 24,
        archive_type: ArchiveType::Texture,
        file_count: 100,
        name_table_offset: 2048
    };

    let serialized = serde_json::to_string(&header).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: crate::header::BA2Header = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    assert_eq!(header.id, deserialized.id);
    assert_eq!(header.version, deserialized.version);
    assert_eq!(header.archive_type, deserialized.archive_type);
    assert_eq!(header.file_count, deserialized.file_count);
    assert_eq!(header.name_table_offset, deserialized.name_table_offset);
}