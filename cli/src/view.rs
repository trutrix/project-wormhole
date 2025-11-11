
#[derive(clap::Subcommand, Debug)]
pub enum ViewMode {
    /// View summary information
    Summary {

    },
    /// View specific record by Editor ID
    Record {
        /// Editor ID of the record to view
        #[arg(short, long)]
        editor_id: String,
    }
}