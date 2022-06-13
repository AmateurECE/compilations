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
mod video_box;
mod view;

use form::{AppForm, AppFormModel};
use view::{AppView, AppViewModel, ApplicationData};

// Frontend top-level application component
enum Switch {
    Form(AppFormModel),
    View(AppViewModel),
}

enum ModelMessage {
    StartApp(ApplicationData),
}

struct Model {
    switch: Switch,
}

impl Component for Model {
    type Message = ModelMessage;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        Self {
            switch: Switch::Form(
                AppFormModel {
                    callback: context.link().callback(|data: ApplicationData| {
                        ModelMessage::StartApp(data)
                    })
                }),
        }
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        match message {
            ModelMessage::StartApp(data) => {
                self.switch = Switch::View(AppViewModel {
                    data,
                });
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            if let Switch::Form(form) = &self.switch {
                <AppForm ..form.clone() />
            } else if let Switch::View(view) = &self.switch {
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
