use clap::{Parser, Subcommand};
use project_wormhole_esm::esm::{self, MappedESM};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ESM file
    #[arg(short, long)]
    esm_path: String,

    #[command(subcommand)]
    command: Option<TopCommands>
}

#[derive(Subcommand, Debug)]
pub enum TopCommands {
    Benchmark
}

fn main() {
    let args = Args::parse();
    let path = std::path::Path::new(&args.esm_path);
    
    if !path.exists() {
        eprintln!("File does not exist: {}", args.esm_path);
        std::process::exit(1);
    } else if path.is_dir() {
        panic!("Directories not supported yet");
    }

    match args.command {
        Some(TopCommands::Benchmark) => {
            println!("");
            println!("Running benchmark...");
            println!("");

            let file_start = std::time::Instant::now();
            let data = std::fs::read(&args.esm_path).expect("Failed to read file");
            let file_duration = file_start.elapsed();

            let parse_start = std::time::Instant::now();
            let (_, esm) = esm::ESMFull::parse(&data).expect("Error parsing ESM file.");
            let parse_duration_single = parse_start.elapsed();

            let parse_start = std::time::Instant::now();
            let (_, esm) = esm::ESMFull::parse_mt(&data).expect("Error parsing ESM file.");
            let parse_duration_multi = parse_start.elapsed();

            let map_start = std::time::Instant::now();
            let _map = MappedESM::from(esm);
            let map_duration = map_start.elapsed();

            println!("File read time: {:?}", file_duration);
            println!("Parse time (single): {:?}", parse_duration_single);
            println!("Parse time (multi): {:?}", parse_duration_multi);
            println!("Mapping time: {:?}", map_duration);
        },

        None => {
            println!("No command specified");
        }
    }
    

}
