///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Application main view.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/16/2022
////

use yew::prelude::*;
use crate::video_box::VideoBox;

#[derive(Clone, Default, PartialEq)]
pub struct ApplicationData {
    pub username: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AppViewModel {
    pub data: ApplicationData,
}

pub struct AppView;
impl Component for AppView {
    type Message = ();
    type Properties = AppViewModel;

    fn create(_context: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _context: &Context<Self>, _message: Self::Message) ->
        bool
    { false }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <main>
                <div class="video-player flex-space-between">
                    <VideoBox url={"".to_string()} />
                    <VideoBox url={"".to_string()} />
                </div>
            </main>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
