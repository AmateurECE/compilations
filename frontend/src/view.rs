///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Application main view.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/13/2022
////

use yew::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub struct ApplicationData;

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

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    { false }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <p>{ "ModelView" }</p>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
