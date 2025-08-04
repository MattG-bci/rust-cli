mod cli;
mod docs;
use clap::Parser;
use lopdf::Document;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();

    let doc = Document::load(&cli_params.path_to_file).expect("Loading doc failed");
    let pdf_text: Vec<String> = docs::convert_pdf_to_text(&doc);

    let response = cli::generate_response(&cli_params, &ollama, &pdf_text);


}
