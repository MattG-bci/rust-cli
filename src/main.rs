mod cli;
mod docs;
use clap::Parser;
use ollama_rs::Ollama;


#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();
    let response = cli::generate_response(&cli_params, &ollama);

    println!("{}", response.await.response);

    let pdf_file = docs::convert_pdf_to_text(cli_params.path_to_file);
}
