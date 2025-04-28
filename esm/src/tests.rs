use crate::{dev::*};
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

    for g in esm.data {
        match &g.header.label {
            GroupLabel::Top(four_cc) => {
                if &four_cc.0 == b"CELL" {
                    println!("{:#?}", g.data)
                }
            }

            _ => { }
        }
    }

    println!("Time to load: {:?}", start.elapsed());

    println!("Parsed ESM: {:?}", esm.header);

}