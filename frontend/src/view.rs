///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Application main view.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/21/2022
////

use core::cmp::min;
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
    VideoEnded(Callback<Option<Post>>),
}

#[derive(Default)]
pub struct AppView {
    // Mechanism to retrieve new posts
    post_collection: Option<PostCollection>,

    // List of filtered posts
    post_list: Option<VecDeque<Post>>,

    // List of children waiting for a post
    wait_queue: VecDeque<Callback<Option<Post>>>,
}

impl AppView {
    fn update_collection(&self, context: &Context<Self>) {
        use AppViewMessage::*;
        if let Some(collection) = self.post_collection.as_ref() {
            let link = context.link().callback(
                |(value, collection)| ReceivedList((value, collection)));
            let mut collection = collection.clone();
            spawn_local(async move {
                let response = collection.next().await.unwrap();
                link.emit((response, collection));
            });
        }
    }

    fn wake_wait_queue(&mut self) {
        let post_list = self.post_list.as_mut().unwrap();
        let posts = min(post_list.len(), self.wait_queue.len());
        for _ in 0..posts {
            let callback = self.wait_queue.pop_front().unwrap();
            callback.emit(post_list.pop_front());
        }
    }
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

    fn update(&mut self, context: &Context<Self>, message: Self::Message) ->
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

                self.post_list = Some(post_list);
                self.wake_wait_queue();
                true
            },

            VideoEnded(callback) => {
                let option = self.post_list.as_mut().unwrap().pop_front();
                if let Some(post) = option {
                    callback.emit(Some(post));
                } else {
                    self.wait_queue.push_back(callback);
                    self.update_collection(context);
                }
                false
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        use AppViewMessage::*;
        let first_loop = context.link().callback(|c| VideoEnded(c));
        let second_loop = context.link().callback(|c| VideoEnded(c));

        let unsave = !context.props().data.debug;
        html! {
            if let Some(_) = &self.post_list {
                <main>
                    <div class="video-player">
                        <VideoBox onended={first_loop} unsave={unsave} />
                        <VideoBox onended={second_loop} unsave={unsave} />
                    </div>
                </main>
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
