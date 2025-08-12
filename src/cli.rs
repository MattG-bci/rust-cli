use clap::Parser;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::{request, GenerationResponse};
use request::GenerationRequest;
use serde_yaml;

#[derive(Parser)]
pub struct CliCommand {
    pub command: String,
    pub model: String,
    pub path_to_file: String
}

fn get_prompt_initial_message(command: &String) -> String {
    let lower_command = command.to_lowercase();
    let file = std::fs::File::open("src/prompts/prompts.yaml").expect("Prompt file missing");
    let prompts: serde_yaml::Value = serde_yaml::from_reader(file).expect("Prompts file invalid");

    let fit_prompt = prompts.get(&lower_command).and_then(|x| x.as_str());
    match fit_prompt {
        Some(fit_prompt) => fit_prompt.to_string(),
        None => panic!("Did not find prompt for the provided command: {}", command)
    }
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

pub async fn generate_response(params: &CliCommand, ollama: &Ollama, text: String) -> GenerationResponse {
    let initial_message = get_prompt_initial_message(&params.command);
    let prompt = concat_text_file_and_command(&initial_message, &text);
    let response = ollama
        .generate(
            GenerationRequest::new(params.model.clone(), prompt)
        )
        .await
        .expect("Generating messages failed");
    response
}

#[cfg(test)]
mod tests {
    use crate::cli;
    #[test]
    fn test_concat_test_file_and_command() -> () {
        let str1 = "string1".to_string();
        let str2 = "string2".to_string();

        let out = cli::concat_text_file_and_command(&str1, &str2);
        let expected = String::from("string1\nstring2");

        assert_eq!(out, expected);
    }

    #[test]
    fn test_get_prompt_initial_message() -> () {
        let command = "summarise".to_string();
        let prompt = cli::get_prompt_initial_message(&command);
        assert_eq!(prompt, "You are the most efficient writer. Summarise this document the best way possible. Keep it concise but include crucial details.");
    }

}

