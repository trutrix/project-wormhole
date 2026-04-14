#![allow(unused)]
use std::{collections::{HashMap, HashSet}, fs::{DirEntry, File}, path::PathBuf, str::FromStr};

use comfy_table::presets::UTF8_FULL;

use crate::{esm::{full::ESMFull, mapped::ESMMapped, raw::{ESMRaw}}, records::all::*, structs::chunk::get_file_chunks};


const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";
const FO4_DATA_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data";
const FNV_DATA_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout New Vegas\\Data";
const DUMP_TARGET: &str = "ccBGSFO4110-WS_Enclave.esl";

const TARGET_EXTS: [&str;3] = ["esl", "esp", "esm"];

#[test]
fn test_all_fo4() {
    use std::io::Read;

    let entries = get_targets_in_dir(FO4_DATA_DIR);
    let mut table = comfy_table::Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["File", "Benchmark", "Object Count"]);

    for entry in entries {

        if let Ok(mut file) = File::open(entry.path()) {

            let mut buf = Vec::new();

            if let Ok(file_size) = file.read_to_end(&mut buf) {

                let start = std::time::Instant::now();

                if let Ok(esm) = ESMRaw::parse_as_objects(&buf, 1) {
                    let end = start.elapsed();

                    table.add_row(vec![
                        entry.path().file_name().unwrap().to_str().unwrap(),
                        format!("{:?}", end).as_str(),
                        format!("{:?}", esm.1.header.get_object_count().unwrap_or(&0)).as_str()
                    ]);
                
                } else {
                    table.add_row(vec![
                        entry.path().file_name().unwrap().to_str().unwrap(),
                        "",
                        "Failure"
                    ]);
                }

            } else {
                println!("Could not READ file: {:?}", entry.path().file_name());
            }

        } else {
            println!("{:?}", entry);
        }
    }
    println!("{}", table);
}



#[test]
fn test_all_fnv() {
    use std::io::Read;

    let entries = get_targets_in_dir(FNV_DATA_DIR);

    for entry in entries {

        if let Ok(mut file) = File::open(entry.path()) {

            let mut buf = Vec::new();

            if let Ok(file_size) = file.read_to_end(&mut buf) {

                let start = std::time::Instant::now();

                if let Ok(esm) = ESMRaw::parse_as_objects(&buf, 1) {
                    println!("Parse success: {:?} in {:?} - Object count: {:?} - Map length: {:?}", entry.path().file_name().unwrap(), start.elapsed(), esm.1.header.get_object_count().unwrap_or(&0), esm.1.objects.len());
                } else {
                    println!("Parse failure: {:?}", entry.path().file_name());
                }
                
            } else {
                println!("Could not READ file: {:?}", entry.path().file_name());
            }

        } else {
            println!("{:?}", entry);
        }
    }
}

// #[test]
// #[ignore = "run when needed"]
// fn dump_target() {

    
//     let path = format!("{}/{}", FO4_DATA_DIR, DUMP_TARGET);
//     print!("Dumping: {:?}", path);
//     let file = std::fs::read(path).unwrap();

//     let esm = ESMRaw::parse_v2(&file, 1).unwrap().1;

//     println!("{:#?}", esm);

// }


fn get_targets_in_dir(path: &str) -> Vec<DirEntry> {
    let mut filtered = Vec::new();
    let entries = std::fs::read_dir(path).expect(format!("Could not read directory: {}", path).as_str());

    for entry in entries {
        if let Ok(de) = entry {
            if de.path().extension().is_some_and(|f| {
                TARGET_EXTS.contains(&f.to_str().unwrap())
            }) {
                filtered.push(de);
            }
        }
    }

    filtered
}