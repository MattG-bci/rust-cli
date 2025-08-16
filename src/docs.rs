use std;

pub enum FileType {
    HTML,
    PDF,
    DOC,
    MD,
    TXT,
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
            FileType::DOC => {
                convert_doc_to_html(path_to_file);
            }
            FileType::MD => {}
            FileType::TXT => {}
        }
    }
}

fn save_html_file(path_to_file: &String) -> () {
    std::fs::copy(path_to_file, "./src/.html/outputs.html").unwrap();
}

pub fn identify_file_format(path_to_file: &String) -> FileType {
    let file_extension = path_to_file.split('.').last().unwrap();
    match file_extension {
        "pdf" => FileType::PDF,
        "html" => FileType::HTML,
        "doc" => FileType::DOC,
        _ => panic!("Unsupported file extension: {}", file_extension),
    }
}

fn convert_doc_to_html(path_to_file: &String) -> () {
    if !std::path::Path::new("./src/.html").exists() {
        std::fs::create_dir("./src/.html").unwrap();
    }
    std::process::Command::new("soffice")
        .args([
            "--convert-to",
            "html",
            "--outdir",
            "./src/.html",
            path_to_file,
        ])
        .output()
        .expect("Failed to execute soffice convert");

    let filename = strip_file_name_from_path(path_to_file);
    std::fs::rename(
        format!("./src/.html/{}.html", filename),
        "./src/.html/outputs.html",
    )
    .unwrap()
}

fn convert_pdf_to_html(path_to_file: &String) -> () {
    if !std::path::Path::new("./src/.html").exists() {
        std::fs::create_dir("./src/.html").unwrap();
    }

    std::process::Command::new("pdftohtml")
        .arg(path_to_file)
        .arg("./src/.html/output.html")
        .output()
        .expect("Failed to execute pdftohtml");
}

pub fn convert_html_to_text(path_to_file: &str) -> String {
    std::fs::read_to_string(path_to_file).unwrap()
}

pub fn strip_file_name_from_path(path_to_file: &String) -> &str {
    path_to_file
        .split("/")
        .last()
        .unwrap()
        .split(".")
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::docs;
    #[test]
    fn test_strip_file_name_from_path() {
        let path = "./usr/docs/test_document.pdf".to_string();
        let res = docs::strip_file_name_from_path(&path);
        assert_eq!(res, "test_document");
    }
}
