use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ESM file
    #[arg(short, long)]
    path: String,


}

fn main() {
    let args = Args::parse();

    let path = std::path::Path::new(&args.path);
    if !path.exists() {
        eprintln!("File does not exist: {}", args.path);
        std::process::exit(1);
    } else {
        println!("Loading ESM file: {}", args.path);
    }
}
