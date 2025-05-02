use crate::{dev::*, traits::EditorId};
use std::io::Read;
const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";

#[test]
fn test1() {
    use crate::esm::*;

    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    // let (mut hbuf, mut dbuf) = ESM1::create_buffers();
    let start = std::time::Instant::now();
    // let mut esm = ESM1::new(ESM_PATH, &mut hbuf, &mut dbuf).unwrap();
    let (i, esm) = RawESM::parse(&buf).unwrap();

    // for g in esm.data {
    //     println!("{:?}", g.header.label)
    // }

    // println!("{:#?}", esm.worlds);

    println!("Time to load: {:?}", start.elapsed());

    for wg in esm.worlds {
        for world in wg.worlds {
            println!("{:?}", world.edid());
        }
    }

    
    println!("Parsed {} records", esm.records.len());

}