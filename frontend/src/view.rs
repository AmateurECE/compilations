///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Application main view.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/20/2022
////

use std::collections::VecDeque;

use js_sys::Array;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::api::PostCollection;
use crate::filter::Post;
use crate::video_box::VideoBox;

#[derive(Clone, Default, PartialEq)]
pub struct ApplicationData {
    // Username for requests to Reddit API
    pub username: String,

    // Debug mode
    pub debug: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AppViewModel {
    pub data: ApplicationData,
}

pub enum AppViewMessage {
    ReceivedList((Array, PostCollection)),
    FirstEnded(Callback<Option<Post>>),
    SecondEnded(Callback<Option<Post>>),
}

#[derive(Default)]
pub struct AppView {
    post_collection: Option<PostCollection>,
    post_list: Option<VecDeque<Post>>,
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
            ReceivedList((array, collection)) => {
                self.post_collection = Some(collection);
                let mut post_list = VecDeque::new();
                for value in array.values() {
                    let value = value.unwrap();
                    if let Some(post) = Post::from_object(value) {
                        post_list.push_back(post);
                    }
                }

                self.first_post = post_list.pop_front();
                self.second_post = post_list.pop_front();
                self.post_list = Some(post_list);
                true
            },

            FirstEnded(callback) => {
                callback.emit(self.post_list.as_mut().unwrap().pop_front());
                false
            },

            SecondEnded(callback) => {
                callback.emit(self.post_list.as_mut().unwrap().pop_front());
                false
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        use AppViewMessage::*;
        let first_loop = context.link().callback(|c| FirstEnded(c));
        let second_loop = context.link().callback(|c| SecondEnded(c));

        let unsave = !context.props().data.debug;
        html! {
            if let Some(_) = &self.post_list {
                <main>
                    <div class="video-player flex-space-between">
                        <VideoBox post={self.first_post.clone()}
                         onended={first_loop} unsave={unsave} />
                        <VideoBox post={self.second_post.clone()}
                         onended={second_loop} unsave={unsave} />
                    </div>
                </main>
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
