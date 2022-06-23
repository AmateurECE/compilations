///////////////////////////////////////////////////////////////////////////////
// NAME:            api.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Semantic interface for interacting with the API.
//
// CREATED:         06/16/2022
//
// LAST EDITED:     06/23/2022
////

use model::MediaUrlRequest;
use js_sys::{Array, Reflect};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::{JsCast, JsValue};

// TODO: This is bad! I can't figure out how to get this property into the
// application from the build configuration.
const PUBLIC_URL: &'static str = "/compilations";

async fn fetch(request: web_sys::Request) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Convert the response body to JSON.
    assert!(value.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = value.dyn_into()?;
    Ok(JsFuture::from(response.json().unwrap()).await?)
}

///////////////////////////////////////////////////////////////////////////////
// Public API
////

// Get the identity of the currently logged in user
pub async fn get_identity() -> Result<JsValue, JsValue> {
    let endpoint = PUBLIC_URL.to_string() + "/api/v1/me";
    let request = web_sys::Request::new_with_str(&endpoint)?;
    fetch(request).await
}

pub async fn get_subscribed() -> Result<JsValue, JsValue> {
    let endpoint = PUBLIC_URL.to_string() + "/subreddits/mine/subscriber";
    let request = web_sys::Request::new_with_str(&endpoint)?;
    fetch(request).await
}

// Iterate through a collection of saved posts
#[derive(Clone)]
pub struct PostCollection {
    username: String,
    after: Option<String>,
    count: u32,
}

impl PostCollection {
    pub fn new(username: &str) -> Self {
        let username = username.split('/').last().unwrap().to_owned();
        Self {
            username,
            after: None,
            count: 0,
        }
    }

    pub async fn next(&mut self) -> Result<Array, JsValue> {
        if self.after.is_none() && self.count != 0 {
            return Ok(Array::new());
        }

        // Kick off request to get more, adding query params if this isn't our
        // first request.
        let mut url = PUBLIC_URL.to_string() + "/user/" + &self.username
            + "/saved";
        if let Some(after) = &self.after {
            let query = "?after=".to_owned() + after + "&count="
                + &self.count.to_string();
            url += &query;
        }

        // Get response
        let request = web_sys::Request::new_with_str(&url)?;
        let result = fetch(request).await?;

        // Update query params. if 'after' field in the response is null, we've
        // reached the end of the saved list.
        let data = Reflect::get(&result, &"data".into())?;
        let after = Reflect::get(&data, &"after".into())?;
        let children = Array::from(&Reflect::get(&data, &"children".into())?);
        if after.is_null() {
            self.after = None;
        } else {
            self.after = Some(after.as_string().unwrap());
            self.count = self.count + children.length();
        }

        Ok(children)
    }
}

pub async fn get_video(request: MediaUrlRequest) -> Result<String, JsValue> {
    // Send the request as JSON body
    let headers = web_sys::Headers::new()?;
    headers.set("Content-Type", "application/json").unwrap();

    let mut request_init = web_sys::RequestInit::new();
    request_init.headers(&headers.into());
    request_init.method("POST");
    let body = serde_json::to_string(&request)
        .map_err(|e| JsValue::from(e.to_string()))?;
    request_init.body(Some(&body.into()));

    let request_url = PUBLIC_URL.to_string() + "/video";
    let request = web_sys::Request::new_with_str_and_init(
        &request_url, &request_init)?;

    // Send request
    let window = web_sys::window().unwrap();
    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Convert the response body to text.
    assert!(value.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = value.dyn_into()?;
    JsFuture::from(response.text().unwrap()).await?
        .into_serde::<String>()
        .map_err(|e| e.to_string().into())
}

pub async fn unsave(id: &str) -> Result<(), JsValue> {
    let mut request_init = web_sys::RequestInit::new();
    request_init.method("POST");
    let request_url = PUBLIC_URL.to_string() + "/api/unsave?id=" + id;
    let request = web_sys::Request::new_with_str_and_init(
        &request_url, &request_init)?;

    // Send request
    let window = web_sys::window().unwrap();
    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(value.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = value.dyn_into()?;
    match response.ok() {
        true => Ok(()),
        false => Err(response.status_text().into()),
    }
}

///////////////////////////////////////////////////////////////////////////////
