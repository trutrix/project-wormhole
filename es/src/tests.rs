#![allow(unused)]
use std::{collections::{HashMap, HashSet}, fs::{DirEntry, File}, os::raw, path::PathBuf, str::FromStr};

use comfy_table::presets::UTF8_FULL;

use crate::{dev::GroupLabel, es::{full::ESFull, mapped::ESMapped, raw::ESRaw}, records::all::*, structs::{chunk::get_file_chunks, es_object::RawESObject}};


const FO4_ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";
const FO4_DATA_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data";

const FNV_ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout New Vegas\\Data\\FalloutNV.esm";
const FNV_DATA_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout New Vegas\\Data";

const DUMP_TARGET: &str = "ccBGSFO4110-WS_Enclave.esl";




fn test_es_dir(path: &str) {
    use std::io::Read;

    let entries = get_targets_in_dir(path);
    let mut table = comfy_table::Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["File", "Benchmark", "Objects (Header)", "Objects (Parsed)"]);

    for entry in entries {

        if let Ok(mut file) = File::open(entry.path()) {

            let mut buf = Vec::new();

            if let Ok(file_size) = file.read_to_end(&mut buf) {

                let start = std::time::Instant::now();

                if let Ok((_, esm)) = ESRaw::parse_as_objects(&buf, 1) {
                    let end = start.elapsed();

                    table.add_row(vec![
                        entry.path().file_name().unwrap().to_str().unwrap(),
                        format!("{:?}", end).as_str(),
                        format!("{:?}", esm.header.get_object_count().unwrap_or(&0)).as_str(),
                        format!("{}", esm.get_full_object_count()).as_str()
                    ]);
                
                } else {
                    table.add_row(vec![
                        entry.path().file_name().unwrap().to_str().unwrap(),
                        "Failure",
                        "Failure",
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
#[ignore]
fn test_all() {
    test_es_dir(FO4_DATA_DIR);
    test_es_dir(FNV_DATA_DIR);
}


fn get_targets_in_dir(path: &str) -> Vec<DirEntry> {
    let mut filtered = Vec::new();
    let entries = std::fs::read_dir(path).expect(format!("Could not read directory: {}", path).as_str());

    for entry in entries {
        if let Ok(de) = entry {
            if de.path().extension().is_some_and(|f| {
                crate::dev::TARGET_EXTS.contains(&f.to_str().unwrap())
            }) {
                filtered.push(de);
            }
        }
    }

    filtered
}

#[test]
#[ignore]
fn dump_main() {
    let file = std::fs::read(FO4_ESM_PATH).unwrap();
    let esm = ESRaw::parse_as_objects(&file, 1).unwrap().1;

    for o in &esm.objects {
        match o {
            RawESObject::Record(raw_record) => {
                o.print_header_info(0);
            },
            RawESObject::Group(group) => {
                o.print_header_info(0);
                println!("  Objects: {}", o.get_object_count());
                match group.header.label {
                    GroupLabel::Top(four_cc) => {
                        match &four_cc.0 {
                            b"WRLD" => {
                                for w in &group.data {
                                    match w {
                                        RawESObject::Record(raw_record) => {
                                            w.print_header_info(2);
                                        },
                                        RawESObject::Group(group) => {
                                            w.print_header_info(4);
                                            for wc in &group.data {
                                                wc.print_header_info(6);
                                                match wc {
                                                    RawESObject::Record(raw_record) => 
                                                    {
                                                        
                                                    },
                                                    RawESObject::Group(group) => {
                                                        for ecb in &group.data {
                                                            ecb.print_header_info(8);
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            b"CELL" => {

                            }


                            _ => {  }
                        }


                    }

                    _ => { panic!("Uneexpected group encountered.")}
                }
            }
        }
    }

}

#[test]
fn test_esm_full() {
    let data = std::fs::read(FO4_ESM_PATH).unwrap();
    let esm = ESFull::parse_mt(&data).unwrap().1;
}