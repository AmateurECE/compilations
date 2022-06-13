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

#[derive(Clone, Default, PartialEq, Properties)]
pub struct AppFormModel;

pub struct AppForm;
impl Component for AppForm {
    type Message = ();
    type Properties = AppFormModel;

    fn create(_context: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    { false }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <p>{ "ModelForm" }</p>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
