///////////////////////////////////////////////////////////////////////////////
// NAME:            form.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Form for application configuration.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/16/2022
////

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::api::get_identity;
use crate::filter::IdentityFilter;
use crate::view::ApplicationData;

#[derive(Clone, Default, PartialEq, Properties)]
pub struct AppFormModel {
    pub callback: Callback<ApplicationData>,
}

pub enum AppFormMessage {
    Identity(String),
    Start,
}

#[derive(Default)]
pub struct AppForm {
    username: Option<String>,
}

impl Component for AppForm {
    type Message = AppFormMessage;
    type Properties = AppFormModel;

    fn create(context: &Context<Self>) -> Self {
        use AppFormMessage::*;
        let link = context.link().callback(|data| Identity(data));
        spawn_local(async move {
            let identity = get_identity().await.unwrap();
            let username = IdentityFilter::new(identity).username();
            link.emit(username.unwrap());
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
                };
                context.props().callback.emit(data);
                false
            },
            AppFormMessage::Identity(data) => {
                self.username = Some(data.clone());
                web_sys::console::log_1(&data.into());
                true
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            if let Some(username) = &self.username {
                <div>
                    <p class="text">{ {"u/".to_string()} + username }</p>
                    <button onclick={context.link().callback(|e: MouseEvent| {
                        e.prevent_default();
                        AppFormMessage::Start
                    })}>{ "Start" }</button>
                    </div>
            } else {
                <p>{ "Loading..." }</p>
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
