use nom_derive::Parse;




#[test]
fn test_bgsm() {
    let path = std::path::Path::new("C:\\Users\\trutr\\Desktop\\projects\\project-wormhole\\nif\\Materials\\actors\\Deathclaw\\Deathclaw.BGSM");
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();

    use std::io::Read;
    file.read_to_end(&mut buffer).unwrap();
    let (_, bgsm) = crate::BGSM::parse(&buffer).unwrap();
    println!("{:#?}", bgsm);
}