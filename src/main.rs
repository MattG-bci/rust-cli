mod cli;
mod docs;
use clap::Parser;
use lopdf::Document;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();
    let response = cli::generate_response(&cli_params, &ollama);

    println!("{}", response.await.response);

    let doc = Document::load(cli_params.path_to_file).expect("Loading doc failed");

    let pdf_text: Vec<String> = docs::convert_pdf_to_text(&doc);
    println!("Number of pages: {}", pdf_text.len());
    println!("Text from the first page: {}", pdf_text[0])

}
