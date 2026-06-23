mod tests;

use std::io::Write;

use clap::{Parser, ValueEnum};
use project_wormhole_es::es::{mapped::MappedESM, raw::ESRaw};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Path to the first ESM file
    esm1: String,

    /// Path to the second ESM file
    esm2: String,

    #[arg(short, long)]
    /// Turn on extra debug output, which may be useful for comparing specific groups or records
    debug: bool,

    #[arg(long)]
    /// Dump the results to 3 seperate text files
    dump: bool,

    #[arg(short, long)]
    mode: Option<DiffMode>

}

#[derive(Debug, Clone, ValueEnum)]
pub enum DiffMode {
    Full,
    DataOnly,
    ReferenceOnly
}


fn main() {
    let a = Args::parse();

    let path1 = std::path::Path::new(&a.esm1);
    let path2 = std::path::Path::new(&a.esm2);

    // Not sure if this is necessary, since the file reading will fail anyway, but it allows for a more specific error message.
    if !path1.exists() {
        eprintln!("ESM1 not found - {}", a.esm1);
        std::process::exit(1);
    }

    if !path2.exists() {
        eprintln!("ESM2 not found - {}", a.esm2);
        std::process::exit(1);
    }

    let buf1 = std::fs::read(path1).expect("Failed to read ESM1");
    let buf2 = std::fs::read(path2).expect("Failed to read ESM2");

    let all_start = std::time::Instant::now();

    let esm1_start = std::time::Instant::now();
    let (_, esm1) = ESRaw::parse_as_objects(&buf1, 2).expect("Failed to parse ESM1");
    let esm1_end = esm1_start.elapsed();

    let esm2_start = std::time::Instant::now();
    let (_, esm2) = ESRaw::parse_as_objects(&buf2, 2).expect("Failed to parse ESM2");
    let esm2_end = esm2_start.elapsed();

    let map1_start = std::time::Instant::now();
    let map1 = MappedESM::from(esm1);
    let map1_end = map1_start.elapsed();

    let map2_start = std::time::Instant::now();
    let mut map2 = MappedESM::from(esm2);
    let map2_end = map2_start.elapsed();

    let diff_start = std::time::Instant::now();
    let (updated, unchanged, addition) = map1.diff(&mut map2);
    let diff_end = diff_start.elapsed();

    let all_end = all_start.elapsed();

    if a.dump {
        println!("Dumping...");
        let mut out1 = std::fs::File::create("updated.txt").expect("Could not create updated.txt");
        println!("{:?}", out1);
        let mut out2 = std::fs::File::create("unchanged.txt").expect("Could not create unchanged.txt");
        let mut out3 = std::fs::File::create("addition.txt").expect("Could not create addition.txt");

        for i in &updated {
            let so = format!("{:?}\n", i);
            out1.write(so.as_bytes()).unwrap();
        }

        for i in &unchanged {
            let so = format!("{:?}\n", i);
            out2.write(so.as_bytes()).unwrap();
        }

        for i in &addition {
            let so = format!("{:?}\n", i);
            out3.write(so.as_bytes()).unwrap();
        }
    }

    println!("ESM1 Parse: {:?}", esm1_end);
    println!("ESM2 Parse: {:?}", esm2_end);
    println!("MAP1: {:?}", map1_end);
    println!("MAP2: {:?}", map2_end);
    println!("Diff Time: {:?}", diff_end);
    println!("  Updated: {:?}", updated.len());
    println!("  Unchanged: {:?}", unchanged.len());
    println!("  Addition: {:?}", addition.len());
    println!("Full Time: {:?}", all_end);
}

