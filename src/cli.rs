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


pub fn concat_text_file_and_command(cmd: &String, text: &Vec<String>) -> String {
    let mut out = cmd.clone().to_owned();
    for txt in text {
        out.push_str(&txt);
    }
    out
}


pub async fn generate_response(params: &CliCommand, ollama: &Ollama, text: &Vec<String>) -> GenerationResponse {
    let prompt = concat_text_file_and_command(&params.command, text);

    ollama
        .generate(
            GenerationRequest::new(params.model.clone(), prompt)
        )
        .await
        .expect("Generating messages failed")
}



