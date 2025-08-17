use html2md;
use std;

pub enum FileType {
    HTML,
    PDF,
    DOC,
    MARKDOWN,
    TXT,
}

impl FileType {
    pub fn transform_document_text_to_string(&self, path_to_file: &String) -> String {
        let out_path: &str = "./src/.html/outputs.html";
        match self {
            FileType::HTML => {
                save_html_file(path_to_file);
                let html_text = std::fs::read_to_string(out_path).unwrap();
                let markdown_text = html2md::parse_html(&html_text[..]);
                markdown_text
            }
            FileType::PDF => {
                convert_pdf_to_html(path_to_file);
                let html_text = std::fs::read_to_string(out_path).unwrap();
                let markdown_text = html2md::parse_html(&html_text[..]);
                markdown_text
            }
            FileType::DOC => {
                convert_doc_to_html(path_to_file);
                let html_text = std::fs::read_to_string(out_path).unwrap();
                let markdown_text = html2md::parse_html(&html_text[..]);
                markdown_text
            }
            FileType::MARKDOWN => std::fs::read_to_string(out_path).unwrap(),
            FileType::TXT => std::fs::read_to_string(out_path).unwrap(),
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
        std::fs::create_dir(out_path).unwrap();
    }

    std::process::Command::new("pdftohtml")
        .args([path_to_file, format!("{}/outputs.html", out_path).as_str()])
        .output()
        .expect("Failed to execute pdftohtml");
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
