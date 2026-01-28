use std::path::Path;

use clap::Subcommand;



#[derive(Subcommand, Debug)]
pub enum RecordCommands {
    View,
    Extract
}

pub fn handle_record_command(command: &RecordCommands, path: &Path, record_id: Option<u32>) {

}