use clap::{Parser, ValueEnum};
use project_wormhole_esm::esm::diff::ESMDiff;
use project_wormhole_esm::Parse;

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


    let (_, esm1) = ESMDiff::parse(&buf1).expect("Failed to parse ESM1");
    let (_, mut esm2) = ESMDiff::parse(&buf2).expect("Failed to parse ESM2");

    let diff = project_wormhole_esm::esm::diff::get_diff_form_ids(&esm1, &mut esm2);

    diff.print_summary();
}

