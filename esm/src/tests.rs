
const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";

#[test]
fn test1() {
    use crate::esm::*;
    use crate::{dev::*, records::all::GameSetting};
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

    println!("{:?}", esm.quests[0].quests[0].quest);

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