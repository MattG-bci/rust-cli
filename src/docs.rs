use std;



pub fn convert_pdf_to_html(path_to_file: &String) -> std::process::Output {
    if !std::path::Path::new("./src/.html").exists() {
        std::fs::create_dir("./src/.html").unwrap();
    }

    let cmd = std::process::Command::new("pdftohtml")
        .arg(path_to_file)
        .arg("./src/.html/output.html")
    .output().expect("Failed to execute pdftohtml");
    cmd
}

pub fn convert_html_to_text(path_to_file: &str) -> String {
    std::fs::read_to_string(path_to_file).unwrap()
}


