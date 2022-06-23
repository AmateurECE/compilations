///////////////////////////////////////////////////////////////////////////////
// NAME:            form.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Form for application configuration.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/23/2022
////

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::api;
use crate::filter::{IdentityFilter, Subreddit, SubredditFilter};
use crate::view::ApplicationData;

#[derive(Clone, Default, PartialEq, Properties)]
pub struct AppFormModel {
    pub callback: Callback<ApplicationData>,
}

pub struct Identity {
    pub username: String,
    pub subscribed: Vec<Subreddit>,
}

pub enum AppFormMessage {
    ReceivedIdentity(Identity),
    Start,
}

#[derive(Default)]
pub struct AppForm {
    username: Option<String>,
    debug: NodeRef,
}

impl Component for AppForm {
    type Message = AppFormMessage;
    type Properties = AppFormModel;

    fn create(context: &Context<Self>) -> Self {
        use AppFormMessage::*;
        let link = context.link().callback(|data| ReceivedIdentity(data));
        spawn_local(async move {
            let identity = api::get_identity().await.unwrap();
            let subscribed_to = api::get_subscribed().await.unwrap();

            let username = IdentityFilter::new(identity).username();
            let subreddits = SubredditFilter::new(subscribed_to);
            link.emit(Identity {
                username: username.unwrap(),
                subscribed: subreddits.get(),
            });
        });

        Self::default()
    }

    fn update(&mut self, context: &Context<Self>, message: Self::Message) ->
        bool
    {
        match message {
            AppFormMessage::Start => {
                let data = ApplicationData {
                    username: self.username.as_ref().unwrap().clone(),
                    debug: self.debug.cast::<HtmlInputElement>().unwrap()
                        .checked(),
                };
                context.props().callback.emit(data);
                false
            },
            AppFormMessage::ReceivedIdentity(data) => {
                self.username = Some(data.username.clone());
                true
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            if let Some(username) = &self.username {
                <div class="p-10">
                    <p class="text">{
                        {"Hello, u/".to_string()} + username + "!"
                    }</p>
                    <div class="input-group">
                        <input id="debug" type="checkbox" name="debug"
                         value="yes" ref={self.debug.clone()} />
                        <label class="text" for="debug">{"Debug Mode"}</label>
                    </div>
                    <button onclick={context.link().callback(|e: MouseEvent| {
                        e.prevent_default();
                        AppFormMessage::Start
                    })}>{ "Start" }</button>
                    </div>
            } else {
                <p class="p-10 text">{ "Loading..." }</p>
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
