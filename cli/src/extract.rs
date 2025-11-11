
#[derive(clap::Subcommand, Debug)]
pub enum ExtractMode {
    /// Every group and record
    All {
        /// Path to output directory
        #[arg(short, long)]
        output: String,
    },

    /// Worldspace heightmaps
    Heightmaps {

    }
}