#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::{esm::{diff::ESMDiff, full::ESMFull, mapped::ESMMapped, raw::{ESMRaw, RawESM}}, records::all::*, structs::chunk::{SmartChunks, get_file_chunks, get_file_chunks2}};


const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";

#[test]
#[ignore = "disabled"]
fn test1() {
    use crate::esm::*;
    use crate::dev::*;
    use std::io::Read;


    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    println!("File loaded into memory in: {:?}", start.elapsed());


    let start = std::time::Instant::now();
    //let esm = SmartESM::parse_complete(&buf).unwrap();
    let (_, esm) = RawESM::parse(&buf).unwrap();
    println!("Parsed esm in: {:?}", start.elapsed());

}


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
    let (_, chunks) = SmartChunks::parse(&buf).unwrap();
    println!("SmartChunks::parse: {:?}", start.elapsed());

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

#[test]
#[ignore = "obsolete"]
fn esm_raw_single() {
    use std::io::Read;

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let start = std::time::Instant::now();
    let (_, esm) = RawESM::parse(&buf).unwrap();
    println!("");
    println!("RawESM (Single Thread): {:?}", start.elapsed());
    println!("RawESM record count: {}", esm.data_map.len());
    println!("RawESM references count: {}", esm.refr_map.len());
}


#[test]
#[ignore = "obsolete"]
fn esm_raw_multi() {
    use std::io::Read;

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let start = std::time::Instant::now();
    let (_, esm) = RawESM::parse_mt(&buf).unwrap();
    println!("");
    println!("RawESM (Multi Thread): {:?}", start.elapsed());
    println!("RawESM record count: {}", esm.data_map.len());
    println!("RawESM references count: {}", esm.refr_map.len());
}


#[test]
fn esm_raw_2() {
    use std::io::Read;

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let start = std::time::Instant::now();
    let (_, esm) = ESMRaw::parse(&buf).unwrap();
    println!("");
    println!("RawESM (Multi Thread): {:?}", start.elapsed());
    println!("RawESM record count: {}", esm.data_map.len());
}