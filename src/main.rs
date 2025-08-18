mod api;
mod cli;
mod docs;

use api::api::post_llm_response;
use clap::Parser;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();

    let file_type = docs::identify_file_format(&cli_params.path_to_file);
    let markdown_text = file_type.transform_document_text_to_string(&cli_params.path_to_file)?;
    let llm_response = cli::generate_response(&cli_params, &ollama, markdown_text);
    let file_name = docs::strip_file_name_from_path(&cli_params.path_to_file);
    Ok(post_llm_response(&file_name, llm_response.await.response).await)
}
