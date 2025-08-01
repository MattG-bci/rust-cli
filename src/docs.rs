use lopdf::Document;

pub fn convert_pdf_to_text(doc: &Document) -> Vec<String> {
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
    texts

}


