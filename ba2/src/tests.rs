#![allow(unused_imports)]
use std::{path::PathBuf, str::FromStr};

use super::prelude::*;

#[test]
pub fn test_general_archive() {
    let path = "D:\\SteamLibrary\\steamapps\\common\\Fallout 4\\Data\\Fallout4 - Meshes.ba2";
    let mut ba2 = BA2Archive::open(path).unwrap();
    println!("{:?}", ba2);

    let all_files = ba2.read_all_files();
    println!("Files: {}", all_files.len());
    //println!("{:?}", archive.files);
    //println!("{:?}", archive.files);
}


#[test]
fn test_texture_archive() {
    use std::io::Write;
    let path: &str = "D:\\SteamLibrary\\steamapps\\common\\Fallout 4\\Data\\Fallout4 - Textures4.ba2";
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
fn test_archive_dir() {
    use std::io::Write;
    let path = PathBuf::from_str("D:\\SteamLibrary\\steamapps\\common\\Fallout 4\\Data").unwrap();
    let ba2 = BA2ArchiveGroup::open_all(path).unwrap();
}

/*
#[test]
fn test_archive_group() {
    const PATH: &str = "D:\\SteamLibrary\\steamapps\\common\\Fallout 4\\Data";
    let mut file = TextureArchiveGroup::open_dir(PATH).unwrap();
    println!("Archives len: {}", file.archives.len());
    let img = file.read_to_image("Textures\\Actors\\Alien\\Alien_01_D.DDS").unwrap();
    img.save("Alien_01_Diffuse.png").unwrap();
}
*/