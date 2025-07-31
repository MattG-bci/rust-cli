use lopdf::Document;

pub fn convert_pdf_to_text(path: &String) -> Result<Vec<String>, std::io::Error> {
    let doc = Document::load(path);

    match doc {
        Ok(doc) => {
           let pages = doc.get_pages();
            let mut texts = Vec::new();

            for (i, _) in pages.iter().enumerate() {
                let page_number = i as u32;
                let text = doc.extract_text(&[page_number]);
                match text {
                    Ok(text) => {
                        texts.push(text);
                    }
                    Err(e) => {}
                }
            }
            Ok(texts)
        },
        Err(e) => panic!("{}", e)
    }

}


