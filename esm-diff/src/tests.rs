use project_wormhole_esm::esm::raw::ESMRaw;

const FO4_MASTER: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";



#[test]
fn test_same() {

    let mut file = std::fs::read(FO4_MASTER).unwrap();

    let esm1 = ESMRaw::parse_mt(&mut file).unwrap().1;
    let esm2 = ESMRaw::parse_mt(&mut file).unwrap().1;
    


}