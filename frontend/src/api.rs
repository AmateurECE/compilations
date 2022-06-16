///////////////////////////////////////////////////////////////////////////////
// NAME:            api.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Semantic interface for interacting with the API.
//
// CREATED:         06/16/2022
//
// LAST EDITED:     06/16/2022
////

use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::{JsCast, JsValue};

// TODO: This is bad! I can't figure out how to get this property into the
// application from the build configuration.
const PUBLIC_URL: &'static str = "/compilations";

pub async fn get_identity() -> Result<JsValue, JsValue> {
    let endpoint = PUBLIC_URL.to_string() + "/api/v1/me";
    let request = web_sys::Request::new_with_str(&endpoint)?;
    let window = web_sys::window().unwrap();
    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Convert the response body to JSON.
    assert!(value.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = value.dyn_into()?;
    Ok(JsFuture::from(response.json().unwrap()).await?)
}

///////////////////////////////////////////////////////////////////////////////
