mod cli;

use std;

fn main() {
    let model: String = std::env::args().nth(1).expect("No model specified");
    let path_to_file: String = std::env::args().nth(2).expect("No path to file specified");
    let command: String = std::env::args().nth(3).expect("No specific command specified");

    let cli_params: cli::CliCommand = cli::CliCommand::new(command, model, path_to_file);
    println!("{:?}", cli_params);
}
