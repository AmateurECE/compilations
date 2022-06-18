///////////////////////////////////////////////////////////////////////////////
// NAME:            video_box.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A component for each video box.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/18/2022
////

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::*;
use yew::prelude::*;
use crate::filter::Post;

#[derive(PartialEq, Properties)]
pub struct VideoBoxProperties {
    pub post: Option<Post>,
}

pub enum VideoBoxMessage {
    ReceivedVideoUrl(String),
}

#[derive(Default)]
pub struct VideoBox(Option<String>);

impl VideoBox {
    fn update_video_url(context: &Context<Self>) {
        use VideoBoxMessage::*;
        if let Some(post) = &context.props().post {
            let link = context.link().callback(|url| ReceivedVideoUrl(url));
            let post = post.clone();
            spawn_local(async move {
                match post.get_url().await {
                    Ok(url) => link.emit(url),
                    Err(e) => web_sys::console::error_3(
                        &e,
                        &"while getting post video url".into(),
                        &JsValue::from_serde(&post).unwrap()
                    ),
                };
            });
        }
    }
}

impl Component for VideoBox {
    type Message = VideoBoxMessage;
    type Properties = VideoBoxProperties;

    fn create(context: &Context<Self>) -> Self {
        VideoBox::update_video_url(context);
        Self::default()
    }

    fn changed(&mut self, context: &Context<Self>) -> bool {
        VideoBox::update_video_url(context);
        true
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use VideoBoxMessage::*;
        match message {
            ReceivedVideoUrl(url) => {
                self.0 = Some(url);
                true
            },
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <div class="short-video-box">
                if let Some(url) = self.0.as_ref() {
                    <video controls=true>
                        <source src={url.clone()} />
                    </video>
                }
            </div>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
