#![allow(unused)]
use std::io::prelude::*;
use esm::esm::RawESM;
use nom_derive::*;

fn main() {
    
    let mut file = std::fs::File::open("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm").unwrap();
    
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let (_, esm) = RawESM::parse(&buf).unwrap();
    //std::fs::create_dir("./out").unwrap();

    let mut mod_file = std::fs::File::create("./out/mod.rs").unwrap();
    let mut gfile = std::fs::File::create("./out/group.rs").unwrap();

    // for (form_id, g) in esm.records {
    //     match g.header.label {
    //         esm::structs::group::GroupLabel::Top(label) => {
    //             println!("Attempting to create: {}", label);
    //             let mut out_file = std::fs::File::create(format!("./out/{}.rs", label)).unwrap();
    //             out_file.write_all(format!("use crate::dev::*;\n\n").as_bytes()).unwrap();

    //             out_file.write_all(format!("define_record2! {{\n    b\"{}\",\n    {}, [\n    ]\n}}", label, label).as_bytes()).unwrap();

    //             mod_file.write_all(format!("pub mod {};\n", label).as_bytes()).unwrap();

    //             gfile.write_all(format!("{}({}),\n", label, label).as_bytes()).unwrap();
    //         }
    //         _ => {

    //         }
    //     }
        
    // }

}