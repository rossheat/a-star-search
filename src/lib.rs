use fluvio_wasm_timer::Delay;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys::{Document, Element, HtmlParagraphElement, Text, console, window};

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
pub async fn run() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p = document.create_element("p")?;
    p.set_text_content(Some("Hello from Rust!"));
    body.append_child(&p)?;

    let dur = Duration::from_millis(500);
    Delay::new(dur).await;

    // --- delay of 5 seconds goes here ---

    let p = document.create_element("p")?;
    p.set_text_content(Some("It's a good day!"));
    body.append_child(&p)?;

    Ok(())
}
