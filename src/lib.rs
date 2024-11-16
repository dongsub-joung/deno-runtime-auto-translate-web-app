// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

use reqwest::{Body, Error};
use serde::{Serialize, Deserialize};


// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn add(sentence: String) -> String {
  let mut translated= String::new();
  
  let data= post_send_request(sentence);

  let unwraped= data.unwrap();

  // translated= data

  return translated;
}



#[derive(Serialize, Deserialize)]
struct MyData {
    body: String,
}

#[tokio::main]
async fn post_send_request(sentence: String) -> Result<String, reqwest::Error> {
    // The URL to which you are sending the POST request
    let url = "https://httpbin.org/post";

    // Data to send in the request body
    let data = MyData {
        body: sentence,
    };

    // Send the POST request with JSON body
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&data)  // Send the data as JSON
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Request successful!");
        return response.text().await;
        
    } else {
        println!("Request failed with status: {}", response.status());
    }


    Ok(String::new())
}