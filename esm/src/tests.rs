use crate::dev::*;

const ESM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";

#[test]
fn test1() {
    use crate::esm::*;
    let (mut hbuf, mut dbuf) = ESM1::create_buffers();
    let mut start = std::time::Instant::now();
    let mut esm = ESM1::new(ESM_PATH, &mut hbuf, &mut dbuf).unwrap();
    println!("Time to load: {:?}", start.elapsed());

    println!("Header: {:?}", esm.header);

}