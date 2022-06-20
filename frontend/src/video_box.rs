///////////////////////////////////////////////////////////////////////////////
// NAME:            video_box.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A component for each video box.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/20/2022
////

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::HtmlVideoElement;
use yew::prelude::*;
use crate::filter::Post;

#[derive(PartialEq, Properties)]
pub struct VideoBoxProperties {
    pub onended: Callback<Callback<Option<Post>>>,
    pub unsave: bool,
}

pub enum VideoBoxMessage {
    ReceivedVideoUrl(String),
    VideoEnded,
    NewPost(Option<Post>),
}

#[derive(Default)]
pub struct VideoBox {
    post: Option<Post>,
    url: Option<String>,
}

impl VideoBox {
    fn fetch_video_url(&self, context: &Context<Self>) {
        use VideoBoxMessage::*;
        if let Some(post) = &self.post {
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

    fn fetch_next_post(&self, context: &Context<Self>) {
        use VideoBoxMessage::*;
        let callback = context.link().callback(|post| NewPost(post));
        context.props().onended.emit(callback);
    }
}

impl Component for VideoBox {
    type Message = VideoBoxMessage;
    type Properties = VideoBoxProperties;

    fn create(context: &Context<Self>) -> Self {
        let video_box = Self::default();
        video_box.fetch_next_post(context);
        video_box
    }

    fn update(&mut self, context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use VideoBoxMessage::*;
        match message {
            ReceivedVideoUrl(url) => {
                self.url = Some(url);
                true
            },

            VideoEnded => {
                if context.props().unsave {
                    let post = self.post.as_ref().unwrap().clone();
                    spawn_local(async move {
                        post.unsave().await.unwrap();
                    });
                }

                self.fetch_next_post(context);
                true
            },

            NewPost(post) => {
                self.url = None;
                self.post = post;
                self.fetch_video_url(context);
                true
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        use VideoBoxMessage::*;
        let canplaythrough = |e: Event| {
            spawn_local (async move {
                JsFuture::from(e.target_dyn_into::<HtmlVideoElement>()
                               .unwrap().play().unwrap()).await.unwrap();
            });
        };

        html! {
            <div class="player-window">
                if let Some(url) = self.url.as_ref() {
                    <p class="text video-title">{
                        &self.post.as_ref().unwrap().title
                    }</p>
                    <video controls=true oncanplaythrough={canplaythrough}
                     onended={context.link().callback(|_| VideoEnded)}>
                        <source src={url.clone()} />
                    </video>
                }
            </div>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
