use std::{collections::{HashMap, HashSet}, io::Read, path::Path};

use project_wormhole_es::esm::raw::ESMRaw;
// use project_wormhole_esm::esm::raw::RawESM;
use project_wormhole_shared::structs::fourcc::FourCC;


#[deprecated]
pub fn dump_esm_fields(path: &Path) {
    // let mut file = std::fs::File::open(path).expect("Failed to open file");
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf).expect("Failed to read file");
    // let (_, esm) = ESMRaw::parse_mt(&buf).expect("Failed to parse ESM");

    // // Create a nested hashmap to store the dump data in the format:
    // // [ Record Idens: [ Field Idens: [ Field Sizes ] ] ]
    // let mut dump: HashMap<FourCC, HashMap<FourCC, HashSet<u16>>> = HashMap::with_capacity(100000);


    // let mut rcount = 0;

    // for (_rid, rr) in esm.data_map {

    //     let record_entry = dump.entry(rr.header.iden).or_insert(HashMap::new());
    //     let (_, record_fields) = rr.get_raw_fields().expect("Could not convert record data into raw fields.");
    //     rcount += 1;

    //     for field in record_fields {
    //         let field_entry = record_entry.entry(field.header.iden().clone()).or_insert(HashSet::new());
    //         field_entry.insert(field.data.len() as u16);
    //     }
        
    // }

    // let json = serde_json::to_string_pretty(&dump).expect("Failed to serialize dump to JSON");
    // let out_path = path.with_extension("_fields_dump.json");
    // std::fs::write(&out_path, json).expect("Failed to write dump to file");
    // println!("Dumped fields from {} records to {}", rcount, out_path.display());
}