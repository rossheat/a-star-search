use wasm_bindgen::prelude::*; 
use web_sys::{window, console, Document, Element, HtmlParagraphElement, Text};


#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn setup() -> Result<(), JsValue> {
    set_panic_hook();
    console::log_1(&"Successfully set_panic_hook".into());
    Ok(())
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {

    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p = document.create_element("p")?;
    p.set_text_content(Some("Hello from Rust"));
    body.append_child(&p)?;
    
    Ok(())
}
