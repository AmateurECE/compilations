///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Application main view.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/17/2022
////

use js_sys::Array;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::api::PostCollection;
use crate::filter::Post;
use crate::video_box::VideoBox;

#[derive(Clone, Default, PartialEq)]
pub struct ApplicationData {
    pub username: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AppViewModel {
    pub data: ApplicationData,
}

pub enum AppViewMessage {
    ReceivedList((Array, PostCollection)),
}

#[derive(Default)]
pub struct AppView {
    post_collection: Option<PostCollection>,
    video_list: Option<Array>,
    first_post: Option<Post>,
    second_post: Option<Post>,
}

impl Component for AppView {
    type Message = AppViewMessage;
    type Properties = AppViewModel;

    fn create(context: &Context<Self>) -> Self {
        use AppViewMessage::*;
        let link = context.link()
            .callback(|(value, collection)| ReceivedList((value, collection)));
        let username = context.props().data.username.clone();
        spawn_local(async move {
            let mut collection = PostCollection::new(&username);
            let response = collection.next().await.unwrap();
            link.emit((response, collection));
        });

        Self::default()
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use AppViewMessage::*;
        match message {
            ReceivedList((value, collection)) => {
                self.post_collection = Some(collection);
                self.video_list = Some(value);
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            if let Some(_) = &self.video_list {
                <main>
                    <div class="video-player flex-space-between">
                        <VideoBox post={self.first_post.clone()} />
                        <VideoBox post={self.second_post.clone()} />
                    </div>
                </main>
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
