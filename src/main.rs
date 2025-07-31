mod cli;
mod docs;
use clap::Parser;
use ollama_rs::Ollama;
use lopdf::Document;

#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();
    let response = cli::generate_response(&cli_params, &ollama);

    println!("{}", response.await.response);

    let doc = Document::load(cli_params.path_to_file);
    match doc {
        Ok(doc) => {
            let text = doc.extract_text(&[1]).unwrap();
            println!("Text: {:?}", text);
        },
        Err(e) => println!("{}", e)
    }
}
