#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::{records::all::*, structs::world_entry::WorldChildren};


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
pub fn top_group_test() {
    use crate::esm::*;
    use crate::dev::*;
    use std::io::Read;

    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let (i, header) = FileHeader::parse(&buf).unwrap();
    //let (i, e) = WorldChildren::parse(i).unwrap();
    let (i, top_group) = many0(TopGroup::parse)(i).unwrap();

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