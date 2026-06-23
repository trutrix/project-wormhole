#![allow(unused)]

use project_wormhole_es::es::raw::ESRaw;

const FO4_MASTER: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";



#[test]
fn test_same() {

    let mut file = std::fs::read(FO4_MASTER).unwrap();

    let esm1 = ESRaw::parse_as_objects(&mut file, 2).unwrap().1;
    let esm2 = ESRaw::parse_as_objects(&mut file, 2).unwrap().1;
    


}