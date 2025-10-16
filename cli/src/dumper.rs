use std::{collections::{HashMap, HashSet}, io::Read, path::Path};

use esm::structs::fourcc::FourCC;



pub fn dump_esm_fields(path: &Path) {
    let mut file = std::fs::File::open(path).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    let (_, esm) = esm::esm::RawESM::parse(&buf).expect("Failed to parse ESM");

    // Create a nested hashmap to store the dump data in the format:
    // [ Record Idens: [ Field Idens: [ Field Sizes ] ] ]
    let mut dump: HashMap<FourCC, HashMap<FourCC, HashSet<u16>>> = HashMap::with_capacity(100000);

    unimplemented!("Dump ESM fields");
}