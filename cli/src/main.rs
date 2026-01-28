use clap::{Parser, Subcommand};

use crate::{group::GroupCommands, record::{RecordCommands, handle_record_command}};
mod record;
mod group;

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
    Record {
        #[command(subcommand)]
        record_command: RecordCommands,
        #[arg(short)]
        record_id: Option<u32>
    },
    Group {
        #[command(subcommand)]
        group_command: GroupCommands
    }
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
        Some(c) => {
            match c {
                TopCommands::Group { group_command } => todo!(),
                TopCommands::Record { record_command, record_id } => handle_record_command(&record_command, &path, record_id),
            }
        },
        None => {
            eprintln!("No command specified. Use --help for usage information.");
        },
    }

}
