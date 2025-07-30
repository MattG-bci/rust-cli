mod cli;
use clap::Parser;
use ollama_rs::Ollama;


#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();
    let response = cli::generate_response(cli_params, &ollama);

    println!("{}", response.await.response);
}
