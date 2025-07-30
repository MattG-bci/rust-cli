use clap::Parser;
#[derive(Parser)]
pub struct CliCommand {
    pub command: String,
    pub model: String,
    pub path_to_file: String
}
