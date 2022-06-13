///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the frontend application.
//
// CREATED:         06/13/2022
//
// LAST EDITED:     06/13/2022
////

use yew::prelude::*;

mod form;
mod view;

use form::{AppForm, AppFormModel};
use view::{AppView, AppViewModel};

// Frontend top-level application component
enum Model {
    Form(AppFormModel),
    View(AppViewModel),
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self::Form(AppFormModel::default())
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    { false }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            if let Model::Form(form) = &self {
                <AppForm ..form.clone() />
            } else if let Model::View(view) = &self {
                <AppView ..view.clone() />
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Model>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
