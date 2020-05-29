extern crate lopdf;
use lopdf::Document;
use std::path::PathBuf;

pub fn get_pdf_length(path: &PathBuf) -> Result<usize, String> {
    match Document::load(path) {
        Ok(doc) => {
            Ok(doc.get_pages().len())
        },
        Err(_) => return Err("An error occured when parsing the pdf file.".to_owned())
    }
}
