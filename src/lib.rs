use wasm_bindgen::prelude::*;


//Create elements and append it to the HTML body.
pub fn document_create_element()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let title = document.create_element("H1").unwrap();
    let paragraph = document.create_element("p").unwrap();
    let div = document.create_element("div").unwrap();
    
    div.append_child(&title).unwrap();
    div.append_child(&paragraph).unwrap();

    title.set_inner_html("First Rust-WebAssemly program");
    paragraph.set_inner_html("Hello DOM Rust WebAssembly");
    

    body.append_child(&div).unwrap();
}


#[wasm_bindgen(start)]
pub fn start() {
    document_create_element();
}