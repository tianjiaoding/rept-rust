#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_include_static_resources;

extern crate rocket_raw_response;

#[macro_use]
extern crate rocket;

extern crate rocket_multipart_form_data;

extern crate dirs;

use rocket::http::ContentType;
use rocket::Data;

use rocket_include_static_resources::StaticResponse;

use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

use std::fs;

mod date;
mod pdf;
mod name;


#[get("/")]
fn index() -> StaticResponse {
    static_response!("html-pdf-uploader")
}

#[post("/upload", data = "<data>")]
fn upload(content_type: &ContentType, data: Data) -> Result<String, String> {

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec![
            MultipartFormDataField::file("pdf")
                .size_limit(32 * 1024 * 1024)
                .content_type_by_string(Some(mime::APPLICATION_PDF))
                .unwrap(),
            MultipartFormDataField::text("fname"),
        ]
    );

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err("The file is too large.".to_owned())
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err("Please upload a valid pdf.".to_owned())
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let first_name: String;
    match multipart_form_data.texts.remove("fname") {
        Some(mut text_fields) => {
            let text_field = text_fields.remove(0);
            let _text = text_field.text;

            if !name::valid_name_chars(&_text) {
                return Err("Only a-z, A-Z and 0-9 are allowed for the first name.".to_owned());
            }

            if !name::valid_name_len(&_text) {
                return Err("The first name can only be of length 1-20.".to_owned());
            }
            first_name = _text;
        },
        None => return Err("Please input your first name.".to_owned())
    }

    match multipart_form_data.files.get("pdf") {
        Some(file_fields) => {
            let file = &file_fields[0];
            let _content_type = &file.content_type;
            let _file_name = &file.file_name;
            let _path = &file.path;

            println!("{:?}", _path);

            let n_pages = pdf::get_pdf_length(_path)?;
            if n_pages != 1 {
                return Err(format!("Your pdf has {} pages, while exactly 1 is required.", n_pages))
            }

            let mut store_dir = dirs::home_dir().unwrap();
            store_dir.push("research-reports");
            store_dir.push(date::str_next_monday());
            fs::create_dir_all(&store_dir);

            let mut pdf_target_path = store_dir;
            pdf_target_path.push(first_name);
            pdf_target_path.push(".pdf");

            fs::rename(_path, &pdf_target_path);

            return Ok(format!("You are all set! \nPdf has been uploaded to {}", pdf_target_path.to_str().unwrap()))
        },
        None => return Err("Please input a file.".to_owned())
    }
}



fn main() {
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "html-pdf-uploader",
                "front-end/html/pdf-uploader.html",
            );
        }))
        .mount("/", routes![index, upload])
        .launch();
}
