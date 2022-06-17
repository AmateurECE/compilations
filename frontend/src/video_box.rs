///////////////////////////////////////////////////////////////////////////////
// NAME:            video_box.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A component for each video box.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/17/2022
////

use yew::prelude::*;
use crate::filter::Post;

#[derive(PartialEq, Properties)]
pub struct VideoBoxProperties {
    pub post: Option<Post>,
}

pub struct VideoBox;

impl Component for VideoBox {
    type Message = ();
    type Properties = VideoBoxProperties;

    fn create(_context: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _context: &Context<Self>, _message: Self::Message) ->
        bool
    { false }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <div class="short-video-box">
            </div>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
