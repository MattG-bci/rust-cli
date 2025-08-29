use crate::errors;
use clap::Parser;
use ollama_rs::generation::completion::{request, GenerationResponse};
use ollama_rs::Ollama;
use request::GenerationRequest;
use serde_yaml;

#[derive(Parser)]
pub struct LLMCommand {
    pub command: String,
    pub model: String,
    #[arg(long)]
    pub with_thinking: bool,
    pub path_to_file: String,
}

pub fn get_prompt_initial_message(command: &String) -> Result<String, Box<dyn std::error::Error>> {
    let lower_command = command.to_lowercase();
    let file = std::fs::File::open("src/prompts/prompts.yaml")?;
    let prompts: serde_yaml::Value = serde_yaml::from_reader(file)?;

    let fit_prompt = prompts
        .get(&lower_command)
        .and_then(|x| x.as_str())
        .ok_or_else(|| errors::Error::PromptNotFound)?;
    Ok(fit_prompt.to_string())
}

pub fn establish_connection_with_ollama() -> Ollama {
    Ollama::default()
}

pub fn concat_text_file_and_command(cmd: &String, text: &String) -> String {
    let mut out = cmd.clone().to_owned();
    out.push_str("\n");
    out.push_str(&text);
    out
}

pub fn prepare_prompt(
    command: &String,
    text: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let initial_message = get_prompt_initial_message(command)?;
    let prompt = concat_text_file_and_command(&initial_message, &text);
    Ok(prompt)
}

pub async fn generate_response(
    params: &LLMCommand,
    prompt: Result<String, Box<dyn std::error::Error>>,
    ollama: &Ollama,
) -> Result<GenerationResponse, Box<dyn std::error::Error>> {
    let request = GenerationRequest::new(params.model.clone(), prompt?);
    if let true = params.with_thinking {
        let request_with_thinking = request.clone().think(true);
        let response = ollama.generate(request_with_thinking).await?;
        Ok(response)
    } else {
        let response = ollama.generate(request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::llm::cli;
    use crate::llm::cli::prepare_prompt;

    #[test]
    fn test_prepare_prompt() {
        let command = String::from("summarise");
        let md_txt = "Markdown Markdown".to_string();
        let result = prepare_prompt(&command, md_txt).unwrap();
        assert_eq!(result, "You are the most efficient writer. Summarise this document the best way possible. Keep it concise but include crucial details.\nMarkdown Markdown");
    }

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
        let prompt = cli::get_prompt_initial_message(&command).unwrap();
        assert_eq!(prompt, "You are the most efficient writer. Summarise this document the best way possible. Keep it concise but include crucial details.");
    }
}
