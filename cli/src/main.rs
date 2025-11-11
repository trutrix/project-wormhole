mod dumper;
mod report;
mod extract;
mod view;

use clap::{Parser, Subcommand};
use extract::ExtractMode;

use crate::view::ViewMode;

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
    /// Write data to disk
    Extract {
        #[command(subcommand)]
        extract_command: ExtractMode,
        #[arg(short, long)]
        output: String
    },
    /// View data
    View {
        #[command(subcommand)]
        view_command: ViewMode
    }
}

fn main() {
    let args = Args::parse();
    let path = std::path::Path::new(&args.esm_path);
    
    if !path.exists() {
        eprintln!("File does not exist: {}", args.esm_path);
        std::process::exit(1);
    }

    match args.command {
        Some(c) => {
            match c {
                TopCommands::Extract { extract_command, output } => todo!(),
                TopCommands::View { view_command } => todo!(),
            }
        },
        None => {
            eprintln!("No command specified. Use --help for usage information.");
        },
    }

}
