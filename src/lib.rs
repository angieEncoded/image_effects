use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::encode;
use base64::decode;
use image::load_from_memory;
use image::ImageOutputFormat::Png;

// if we are borrowing a string, type is &str
// if we have ownership of a string, use String


#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // log(&encoded_file.into());

    log(&"Grayscale called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    image = image.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    data_url

}