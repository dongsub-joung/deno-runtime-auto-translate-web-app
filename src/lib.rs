use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit, RequestMode, Response, Headers, Fetch};
// the package `deno-runtime-auto-translate-web-app` depends on `web-sys`, with features: `Fetch` but `web-sys` does not have these features.

use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

use serde::{Serialize};
use serde_json::{Value, to_value};


// wasm-pack build --target web
// https://stackoverflow.com/questions/77402053/can-tokioruntime-be-used-in-wasm
#[wasm_bindgen]
pub async fn make_post_request(post_data: &str) -> Result<(), JsValue> {
    // URL to make the request to
    let url = "https://jsonplaceholder.typicode.com/posts";

    // Create the request body as JSON

    // Initialize request options
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&js_sys::JsString::from(post_data).into()));

    // Create the request and set headers
    let request = Request::new_with_str_and_init(url, &opts)?;
    let headers = request.headers();
    headers.set("Content-Type", "application/json")?;

    // Fetch the request and await the response
    let window = web_sys::window().ok_or("no global window object")?;
    let fetch_promise = window.fetch_with_request(&request);

    // Convert the promise to a Rust Future
    let response: Response = JsFuture::from(fetch_promise).await?.dyn_into()?;
    
    // Check if the response status is OK
    if response.ok() {
        web_sys::console::log_1(&"POST request was successful!".into());
        // return responese data (unwraped)
    } else {
        web_sys::console::log_1(&"POST request failed.".into());
    }

    Ok(())
}

#[derive(Serialize)]
struct MyData{
    text: String
}

fn struct_to_json(text_data: &MyData) -> Result<Value, serde_json::Error> {
    // Serialize the struct into a serde_json::Value
    to_value(person)
}
