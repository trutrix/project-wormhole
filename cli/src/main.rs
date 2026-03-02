use clap::{Parser, Subcommand};
use project_wormhole_esm::esm::full::ESMFull;
use project_wormhole_esm::esm::{self, mapped::MappedESM};
use project_wormhole_esm::prelude::Parse;

mod benchmark;
mod diff;
mod dumper;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ESM file
    // #[arg(short, long)]
    // esm_path: String,

    #[command(subcommand)]
    command: TopCommands
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
pub enum TopCommands {
    Benchmark {
        path: String,
    },
    Diff {
        path_a: String,
        path_b: String,

        #[arg(short, default_value_t = 4)]
        /// Number of threads to use for diffing (unused for now)
        threads: usize,

        #[arg(short)]
        /// Whether to generate a report file with the diff results
        report: bool,

        #[arg(short = 'p')]
        /// Path to save the report file (if -r is set)
        report_path: Option<String>,
    }
}

fn main() {
    let args = Args::parse();
    // let path = std::path::Path::new(&args.esm_path);
    
    // if !path.exists() {
    //     eprintln!("File does not exist: {}", args.esm_path);
    //     std::process::exit(1);
    // } else if path.is_dir() {
    //     panic!("Directories not supported yet");
    // }

    // if args.command.is_none() {
    //     panic!("No command specified!");
    // }

    match args.command {
        TopCommands::Benchmark {path} => {
            println!("");
            println!("Running benchmark...");
            println!("");

            let file_start = std::time::Instant::now();
            let data = std::fs::read(&path).expect("Failed to read file");
            let file_duration = file_start.elapsed();

            let parse_start = std::time::Instant::now();
            let (_, esm) = ESMFull::parse(&data).expect("Error parsing ESM file.");
            let parse_duration_single = parse_start.elapsed();

            let parse_start = std::time::Instant::now();
            let (_, esm) = ESMFull::parse_mt(&data).expect("Error parsing ESM file.");
            let parse_duration_multi = parse_start.elapsed();

            let map_start = std::time::Instant::now();
            let _map = MappedESM::from(esm);
            let map_duration = map_start.elapsed();

            println!("File read time: {:?}", file_duration);
            println!("Parse time (single): {:?}", parse_duration_single);
            println!("Parse time (multi): {:?}", parse_duration_multi);
            println!("Mapping time: {:?}", map_duration);
        },

        TopCommands::Diff { path_a, path_b, threads, report, report_path}=> {
            
            let total_start = std::time::Instant::now();

            let path_a = std::path::Path::new(&path_a);
            let path_b = std::path::Path::new(&path_b);

            if !path_a.exists() {
                eprintln!("File does not exist: {}", path_a.display());
                std::process::exit(1);
            } else if path_a.is_dir() {
                panic!("Directories not supported yet");
            }

            if !path_b.exists() {
                eprintln!("File does not exist: {}", path_b.display());
                std::process::exit(1);
            } else if path_b.is_dir() {
                panic!("Directories not supported yet");
            }

            let read_start_a = std::time::Instant::now();
            let esm_file_a = std::fs::read(path_a).expect("Failed to read file A");
            let read_duration_a = read_start_a.elapsed();

            let read_start_b = std::time::Instant::now();
            let esm_file_b = std::fs::read(path_b).expect("Failed to read file B");
            let read_duration_b = read_start_b.elapsed();

            let parse_start_a = std::time::Instant::now();
            let (_, esm_a) = esm::diff::ESMDiff::parse(&esm_file_a).expect("Error parsing ESM file A.");
            let parse_duration_a = parse_start_a.elapsed();

            let parse_start_b = std::time::Instant::now();
            let (_, mut esm_b) = esm::diff::ESMDiff::parse(&esm_file_b).expect("Error parsing ESM file B.");
            let parse_duration_b = parse_start_b.elapsed();

            let start = std::time::Instant::now();
            let result = esm::diff::get_diff_form_ids(&esm_a, &mut esm_b);
            let duration = start.elapsed();
            let total_duration = total_start.elapsed();


            // println!("File A read time: {:?}", read_duration_a);
            // println!("File B read time: {:?}", read_duration_b);
            // println!("File A parse time: {:?}", parse_duration_a);
            // println!("File B parse time: {:?}", parse_duration_b);
            // println!("Diffing took: {:?}", duration);
            // println!("Total time: {:?}", total_duration);


            let mut table = comfy_table::Table::new();
            table.set_header(vec!["Change Type", "Count", "", "Benchmark Name", "Benchmark Time"]);
            table.add_row(vec!["Header Changed", &result.header_changed.to_string(), "", "Parse A", &format!("{:?}", parse_duration_a)]);
            table.add_row(vec!["Additions", &result.additions.len().to_string(), "", "Parse B", &format!("{:?}", parse_duration_b)]);
            table.add_row(vec!["Deletions", &result.deletions.len().to_string(), "", "File Read A", &format!("{:?}", read_duration_a)]);
            table.add_row(vec!["Changed", &result.changed.len().to_string(), "", "File Read B", &format!("{:?}", read_duration_b)]);
            table.add_row(vec!["Same", &result.same.len().to_string(), "", "Diffing", &format!("{:?}", duration)]);
            table.add_row(vec!["", "", "", "Total Time", &format!("{:?}", total_duration)]);


            println!("");
            println!("{}", table);
            println!("");

        }
    }
    

}
