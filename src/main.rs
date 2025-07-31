mod cli;
mod docs;
use clap::Parser;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let cli_params: cli::CliCommand = cli::CliCommand::parse();
    let ollama: Ollama = cli::establish_ollama_connection();
    let response = cli::generate_response(&cli_params, &ollama);

    println!("{}", response.await.response);


    let pdf_text = docs::convert_pdf_to_text(&cli_params.path_to_file);
    match pdf_text {
        Ok(pdf_text) => {
            println!("Number of pages: {}", pdf_text.len());
            println!("Text from the first page: {}", pdf_text[0])
        }
        Err(e) => panic!("{}", e)
    }

}
