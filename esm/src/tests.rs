use std::collections::{HashMap, HashSet};

use crate::records::all::{Armor, ArmorAddon, ArtObject, AttractionRule, BodyPartData, Book, Class, Color, NonPlayerCharacter};


const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";

#[test]
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

    
    let mut field_ids: HashMap<FourCC, HashSet<FourCC>> = HashMap::new();

    for (id, rr) in esm.records {
        

        if rr.header.iden.0 != *b"NPC_" {
            continue;
        } else {
            let set = field_ids.entry(rr.header.iden).or_insert(HashSet::new());
            let tr = NonPlayerCharacter::try_from(rr).unwrap();

            println!("{:#?}", tr);

            for f in tr.fields {
                match f {
                    crate::records::all::NonPlayerCharacterField::Unknown(four_cc) => {
                        set.insert(four_cc);
                    }
                    _ => { /* Ignore known fields */ }
                }
            }
        }

        
    }

    println!("{:?}",field_ids);

    // let out_all = "out_all.json";

    // let json = serde_json::to_string_pretty(&field_ids).unwrap();

    // std::fs::write(out_all, json).unwrap();

    //println!("Parsed esm in: {:?}", start.elapsed());
    //println!("{:?}", esm.header);
    
    //println!("Parsed {} raw records", esm.records.len());

    // for (form_id, r) in esm.records {
    //     match r.header.iden.0.as_ref() {
    //         b"GMST" => {
    //             let gmst = GameSetting::try_from(r).unwrap();
    //             println!("GMST: {:#?}", gmst);
    //         }
    //         _ => {}
    //     }
    // }

}