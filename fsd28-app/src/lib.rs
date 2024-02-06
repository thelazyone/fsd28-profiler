mod components {
    pub mod top_menu;
    pub mod roster_view;
    pub mod units_view;
    pub mod modal;
    pub mod action_tree_view;
}
mod app;
mod shared_messages;
mod model;


// Defining the wrap for the WASM app.

use wasm_bindgen::prelude::*;
use crate::app::App;
use web_sys::window;
#[wasm_bindgen(start)]
pub fn run_app() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let element = document.get_element_by_id("app").expect("no element with id 'app'");

    yew::Renderer::<App>::with_root(element).render();
}
