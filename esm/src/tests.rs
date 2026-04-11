#![allow(unused)]
use std::{collections::{HashMap, HashSet}, fs::File, path::PathBuf, str::FromStr};

use crate::{esm::{diff::ESMDiff, full::ESMFull, mapped::ESMMapped, raw::{ESMRaw}}, records::all::*, structs::chunk::{get_file_chunks, get_file_chunks2}};


const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";
const FO4_DATA_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data";
const FNV_DATA_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout New Vegas\\Data";
const DUMP_TARGET: &str = "ccBGSFO4110-WS_Enclave.esl";



#[test]
pub fn esm_benchmarks() {
    use crate::esm::*;
    use crate::dev::*;
    use std::io::Read;

    println!("");

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("Read file to memory: {:?} - {} bytes", start.elapsed(), buf.len());
    println!("");

    let start = std::time::Instant::now();
    let (_, file_chunks) = get_file_chunks2(&buf).unwrap();
    println!("get_file_chunks2: {:?}", start.elapsed());
    println!("");

    // let start = std::time::Instant::now();
    // let (_, esm) = RawESM::parse(&buf).unwrap();
    // println!("RawESM (Single Thread): {:?}", start.elapsed());
    // println!("RawESM record count: {}", esm.records.len());
    // println!("RawESM references count: {}", esm.references.len());
    //println!("Expected records: {:?}", esm.header.fields);
    //println!("Record count: {}", esm.records.len());

    // println!("");
    // let start = std::time::Instant::now();
    // let (_, esm) = ESMFull::parse(&buf).unwrap();
    // println!("ESMFull (Single Thread): {:?}", start.elapsed());

    // let start = std::time::Instant::now();
    // let (_, esm) = ESMFull::parse_mt(&buf).unwrap();
    // println!("ESMFull (Thread Per Group): {:?}", start.elapsed());

    // println!("");
    // let start = std::time::Instant::now();
    // let esm = ESMMapped::from(esm);
    // println!("MappedESM: {:?}", start.elapsed());
    // println!("Mapped Record count: {}", esm.indices.len());

    // let start = std::time::Instant::now();
    // let diff = ESMDiff::parse(&buf).unwrap().1;
    // println!("ESMDiff: {:?}", start.elapsed());
    // println!("Diff record count: {}", diff.data_records.len());
    // println!("Diff cell count: {}", diff.cells.len());
}



#[test]
#[ignore]
fn esm_full_single() {
    use std::io::Read;

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    println!("");
    let start = std::time::Instant::now();
    let (_, esm) = ESMFull::parse(&buf).unwrap();
    println!("ESMFull (Single Thread): {:?}", start.elapsed());
}


#[test]
#[ignore = "multiple multi-threaded tests contaminate results"]
fn esm_full_multi() {
    use std::io::Read;

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    println!("");
    let start = std::time::Instant::now();
    let (_, esm) = ESMFull::parse_mt(&buf).unwrap();
    println!("ESMFull (Multi Thread): {:?}", start.elapsed());
}

// #[test]
// #[ignore = "obsolete"]
// fn esm_raw_single() {
//     use std::io::Read;

//     let start = std::time::Instant::now();
//     let mut file = std::fs::File::open(ESM_PATH).unwrap();
//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf).unwrap();

//     let start = std::time::Instant::now();
//     let (_, esm) = RawESM::parse(&buf).unwrap();
//     println!("");
//     println!("RawESM (Single Thread): {:?}", start.elapsed());
//     println!("RawESM record count: {}", esm.data_map.len());
//     println!("RawESM references count: {}", esm.refr_map.len());
// }


// #[test]
// #[ignore = "obsolete"]
// fn esm_raw_multi() {
//     use std::io::Read;

//     let start = std::time::Instant::now();
//     let mut file = std::fs::File::open(ESM_PATH).unwrap();
//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf).unwrap();

//     let start = std::time::Instant::now();
//     let (_, esm) = RawESM::parse_mt(&buf).unwrap();
//     println!("");
//     println!("RawESM (Multi Thread): {:?}", start.elapsed());
//     println!("RawESM record count: {}", esm.data_map.len());
//     println!("RawESM references count: {}", esm.refr_map.len());
// }


#[test]
#[ignore = "enable when needed"]
fn esm_raw_2() {
    use std::io::Read;

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let start = std::time::Instant::now();
    let (_, esm) = ESMRaw::parse_v2(&buf, 2).unwrap();
    println!("");
    println!("RawESM (Multi Thread): {:?}", start.elapsed());
    println!("RawESM record count: {}", esm.data_map.len());
}


#[test]
fn test_all_fo4() {
    use std::io::Read;

    let entries = std::fs::read_dir(FO4_DATA_DIR).expect("Could not open directory.");

    for entry in entries {

        if let Ok(de) = entry {

            if de.path().extension().is_some_and(|f| {
                f == "esm" || f == "esp" || f == "esl"
            }) {

            if let Ok(mut file) = File::open(de.path()) {

                let mut buf = Vec::new();

                if let Ok(file_size) = file.read_to_end(&mut buf) {

                    let start = std::time::Instant::now();

                    if let Ok(esm) = ESMRaw::parse_v2(&buf, 2) {
                        println!("Parse success: {:?} in {:?} - Object count: {:?} - Map length: {:?}", de.path().file_name().unwrap(), start.elapsed(), esm.1.header.get_object_count().unwrap_or(&0), esm.1.data_map.len());
                    } else {
                        println!("Parse failure: {:?}", de.path().file_name());
                    }


                } else {
                    println!("Could not READ file: {:?}", de.path().file_name());
                }
            } else {
                println!("Could not OPEN file: {:?}", de.path().file_name());
            }

            } else {
                // Not needed
                //println!("Skipping file: {:?}", de.path().file_name());
            }
        } else {
            println!("{:?}", entry);
        }
    }
}



#[test]
#[ignore = "figure out later"]
fn test_all_fnv() {
    use std::io::Read;

    let entries = std::fs::read_dir(FNV_DATA_DIR).expect("Could not open directory.");

    for entry in entries {

        if let Ok(de) = entry {

            if de.path().extension().is_some_and(|f| {
                f == "esm" || f == "esp" || f == "esl"
            }) {

            if let Ok(mut file) = File::open(de.path()) {

                let mut buf = Vec::new();

                if let Ok(file_size) = file.read_to_end(&mut buf) {

                    let start = std::time::Instant::now();

                    if let Ok(esm) = ESMRaw::parse_v2(&buf, 1) {
                        println!("Parse success: {:?} in {:?}", de.path().file_name(), start.elapsed());
                        println!(" Map length: {:?}", esm.1.data_map.len());
                        //println!("  Plus groups: {}", esm.1.data_map.len() + esm.1.group_counter as usize);
                    } else {
                        println!("Parse failure: {:?}", de.path().file_name());
                    }


                } else {
                    println!("Could not READ file: {:?}", de.path().file_name());
                }
            } else {
                println!("Could not OPEN file: {:?}", de.path().file_name());
            }

            } else {
                // Not needed
                //println!("Skipping file: {:?}", de.path().file_name());
            }
        } else {
            println!("{:?}", entry);
        }
    }
}

#[test]
#[ignore = "run when needed"]
fn dump_target() {

    
    let path = format!("{}/{}", FO4_DATA_DIR, DUMP_TARGET);
    print!("Dumping: {:?}", path);
    let file = std::fs::read(path).unwrap();

    let esm = ESMRaw::parse_v2(&file, 1).unwrap().1;

    println!("{:#?}", esm);

}