mod cli;
mod docs;
use clap::Parser;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();

    let file_type = docs::identify_file_format(&cli_params.path_to_file);
    file_type.convert_file_to_html(&cli_params.path_to_file);
    let html_txt = docs::convert_html_to_text("./src/.html/outputs.html");
    let llm_response = cli::generate_response(&cli_params, &ollama, html_txt);
    println!("{}", llm_response.await.response);
    docs::dump_files();
}
