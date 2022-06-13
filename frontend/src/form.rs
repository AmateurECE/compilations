///////////////////////////////////////////////////////////////////////////////
// NAME:            form.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Form for application configuration.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/13/2022
////

use yew::prelude::*;
use crate::view::ApplicationData;

#[derive(Clone, Default, PartialEq, Properties)]
pub struct AppFormModel {
    pub callback: Callback<ApplicationData>,
}

pub enum AppFormMessage {
    Start,
}

pub struct AppForm;
impl Component for AppForm {
    type Message = AppFormMessage;
    type Properties = AppFormModel;

    fn create(_context: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, context: &Context<Self>, message: Self::Message) ->
        bool
    {
        match message {
            AppFormMessage::Start => {
                context.props().callback.emit(ApplicationData::default());
                false
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={context.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    AppFormMessage::Start
                })}>{ "Start" }</button>
            </div>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
