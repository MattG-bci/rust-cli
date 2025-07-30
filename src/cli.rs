use clap::Parser;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::{request, GenerationResponse};
use request::GenerationRequest;

#[derive(Parser)]
pub struct CliCommand {
    pub command: String,
    pub model: String,
    pub path_to_file: String
}


pub fn establish_ollama_connection() -> Ollama {
    Ollama::default()
}


pub async fn generate_response(params: &CliCommand, ollama: &Ollama) -> GenerationResponse {
    ollama
        .generate(
            GenerationRequest::new(params.model.clone(), params.command.clone())
                .think(true)
        )
        .await
        .expect("Generating messages failed")
}



