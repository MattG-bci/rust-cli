mod cli;
use clap::Parser;

fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    println!("{}", cli_params.model);
}
