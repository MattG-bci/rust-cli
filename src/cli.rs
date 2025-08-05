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


fn concat_text_file_and_command(cmd: &String, text: &String) -> String {
    let mut out = cmd.clone().to_owned();
    out.push_str("\n");
    out.push_str(&text);
    out
}

#[test]
fn test_concat_test_file_and_command() -> () {
    let str1 = "string1".to_string();
    let str2 = "string2".to_string();

    let out = concat_text_file_and_command(&str1, &str2);
    let expected = String::from("string1\nstring2");

    assert_eq!(out, expected);
}

pub async fn generate_response(params: &CliCommand, ollama: &Ollama, text: String) -> GenerationResponse {
    let prompt = concat_text_file_and_command(&params.command, &text);
    let response = ollama
        .generate(
            GenerationRequest::new(params.model.clone(), prompt)
        )
        .await
        .expect("Generating messages failed");
    response
}



