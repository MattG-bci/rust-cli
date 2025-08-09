use std;


pub enum FileType {
    HTML,
    PDF,
}

impl FileType {
    pub fn convert_file_to_html(&self, path_to_file: &String) -> () {
        match self {
            FileType::HTML => {
                save_html_file(path_to_file);
            }
            FileType::PDF => {
                convert_pdf_to_html(path_to_file);
            }
        }
    }
}


pub fn dump_files() -> () {
    let files = std::fs::read_dir("./src/.html/").unwrap();
    for file in files {
        std::fs::remove_file(file.unwrap().path()).unwrap();
    }
}


fn save_html_file(path_to_file: &String) -> () {
    std::fs::copy(path_to_file, "./src/.html//outputs.html").unwrap();
}


pub fn identify_file_format(path_to_file: &String) -> FileType {
    let file_extension = path_to_file.split('.').last().unwrap();
    match file_extension {
        "pdf" => FileType::PDF,
        _ => FileType::HTML,
    }
}


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
