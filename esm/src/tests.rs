use std::collections::HashSet;


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

    let mut acount = 0;
    let mut field_ids: HashSet<FourCC> = HashSet::new();

    for (id, rr) in esm.records {
        match rr.header.iden.0.as_ref() {
            b"AECH" => {
                acount += 1;
                
                let aech = crate::records::AECH::AudioEffectChain::try_from(rr).unwrap();
                
                for f in aech.fields {
                    match f {
                        crate::records::all::AudioEffectChainField::Unknown(four_cc) => {  
                            field_ids.insert(four_cc);
                        }

                        _ => {}
                    }
                }
                

            }
            _ => {}
        }
    }

    println!("{:?}",field_ids);

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