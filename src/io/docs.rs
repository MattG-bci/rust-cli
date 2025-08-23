use html2md;
use std;
use std::io::Error;

#[derive(PartialEq, Debug)]
pub enum FileType {
    HTML,
    PDF,
    DOC,
    MARKDOWN,
    TXT,
}

impl FileType {
    pub fn transform_document_text_to_string(
        &self,
        path_to_file: &String,
    ) -> Result<String, Error> {
        let out_path: &str = "./src/.html/outputs.html";
        let markdown_text: String = match self {
            FileType::HTML => {
                save_html_file(path_to_file)?;
                let html_text = std::fs::read_to_string(out_path)?;
                html2md::parse_html(&html_text[..])
            }
            FileType::PDF => {
                convert_pdf_to_html(path_to_file);
                let html_text = std::fs::read_to_string(out_path)?;
                html2md::parse_html(&html_text[..])
            }
            FileType::DOC => {
                convert_doc_to_html(path_to_file);
                let html_text = std::fs::read_to_string(out_path)?;
                html2md::parse_html(&html_text[..])
            }
            FileType::MARKDOWN => std::fs::read_to_string(out_path)?,
            FileType::TXT => std::fs::read_to_string(out_path)?,
        };
        Ok(markdown_text)
    }
}

fn save_html_file(path_to_file: &String) -> Result<u64, Error> {
    std::fs::copy(path_to_file, "./src/.html/outputs.html")
}

pub fn identify_file_format(path_to_file: &String) -> FileType {
    let file_extension = path_to_file.split('.').last().unwrap();
    match file_extension {
        "pdf" => FileType::PDF,
        "html" => FileType::HTML,
        "doc" => FileType::DOC,
        "md" => FileType::MARKDOWN,
        "txt" => FileType::TXT,
        _ => panic!("Unsupported file extension: {}", file_extension),
    }
}

fn convert_doc_to_html(path_to_file: &String) -> () {
    let out_path: &str = "./src/.html";
    if !std::path::Path::new(out_path).exists() {
        std::fs::create_dir(out_path).unwrap();
    }
    std::process::Command::new("soffice")
        .args(["--convert-to", "html", "--outdir", out_path, path_to_file])
        .output()
        .expect("Failed to execute soffice convert");

    let filename = strip_file_name_from_path(path_to_file);
    std::fs::rename(
        format!("{}/{}.html", out_path, filename),
        format!("{}/outputs.html", out_path),
    )
    .unwrap()
}

fn convert_pdf_to_html(path_to_file: &String) -> () {
    let out_path: &str = "./src/.html";
    if !std::path::Path::new(out_path).exists() {
        std::fs::create_dir(out_path).ok();
    }

    std::process::Command::new("pdftohtml")
        .args([path_to_file, format!("{}/outputs.html", out_path).as_str()])
        .output().ok();
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
    use crate::io::docs;
    use crate::io::docs::{identify_file_format, FileType};

    #[test]
    fn test_strip_file_name_from_path() {
        let path = "./usr/docs/test_document.pdf".to_string();
        let res = docs::strip_file_name_from_path(&path);
        assert_eq!(res, "test_document");
    }

    #[test]
    fn test_identify_file_format() {
        let pdf_path = "root/path.pdf".to_string();
        let html_path = "root/outputs.html".to_string();
        let doc_path = "root/outputs.doc".to_string();

        assert_eq!(FileType::PDF, identify_file_format(&pdf_path));
        assert_eq!(FileType::HTML, identify_file_format(&html_path));
        assert_eq!(FileType::DOC, identify_file_format(&doc_path));
    }
}
