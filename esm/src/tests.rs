#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::{esm::{full::ESMFull, mapped::MappedESM, raw::RawESM}, records::all::*, structs::{chunk::{SmartChunks, get_file_chunks, get_file_chunks2}, world::WorldChildren}};


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

    let start = std::time::Instant::now();
    let (_, esm) = RawESM::parse(&buf).unwrap();
    println!("RawESM (Single Thread): {:?}", start.elapsed());
    println!("RawESM record count: {}", esm.records.len());
    //println!("Expected records: {:?}", esm.header.fields);
    //println!("Record count: {}", esm.records.len());

    let start = std::time::Instant::now();
    let (_, esm) = ESMFull::parse(&buf).unwrap();
    println!("ESMFull (Single Thread): {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let (_, esm) = ESMFull::parse_mt(&buf).unwrap();
    println!("ESMFull (Thread Per Group): {:?}", start.elapsed());


    let start = std::time::Instant::now();
    let esm = MappedESM::from(esm);
    println!("MappedESM: {:?}", start.elapsed());
    println!("Record count: {}", esm.indices.len());

    // let start = std::time::Instant::now();
    // let esm = SmartESM2::load_file(ESM_PATH).unwrap();
    // println!("SmartESM2: {:?}", start.elapsed());

    // println!("Record count: {}", esm.records.len());


    // println!("Counted {:?} chunks", chunks.len());

    //println!("Header: {:?}", header);
    

    // for group in &top_group {
    //     match group {
    //         TopGroup::WRLD(worlds) => {
    //             println!("Worlds group with {:#?} worlds", worlds);
    //         },
    //         _ => {}
    //     }
    // }
}