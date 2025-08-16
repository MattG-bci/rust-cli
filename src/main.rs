mod api;
mod cli;
mod docs;

use api::api::post_llm_response;
use clap::Parser;
use html2md;
use html2md::parse_html;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();

    let file_type = docs::identify_file_format(&cli_params.path_to_file);
    file_type.convert_file_to_html(&cli_params.path_to_file);
    let html_text = docs::convert_html_to_text("./src/.html/outputs.html");
    let markdown_text = parse_html(&html_text[..]);
    let llm_response = cli::generate_response(&cli_params, &ollama, markdown_text);
    let file_name = docs::strip_file_name_from_path(&cli_params.path_to_file);
    post_llm_response(&file_name, llm_response.await.response).await;
}
