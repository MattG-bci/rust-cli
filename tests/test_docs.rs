use lopdf::Document;
use rust_cli::docs::convert_pdf_to_text;


fn load_mock_document() -> Document {
    let doc = Document::load("tests/fixtures/mock_pdf.pdf").unwrap();
    doc
}


#[test]
fn test_convert_pdf_to_text() -> () {
    let mock_document = load_mock_document();
    let result = convert_pdf_to_text(&mock_document);
    assert_eq!(result.len(), 12);
}