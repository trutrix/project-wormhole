use clap::Parser;
mod heightmap;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Function to perform
    #[arg(short, long)]
    function: String,
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
    }

    match args.function.as_str() {

        "extract-heightmap" => {
            heightmap::extract_heightmap(path);
        }

        _ => {
            eprintln!("Invalid command");
            std::process::exit(1);
        }
    }

}
