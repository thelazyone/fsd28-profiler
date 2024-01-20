use yew::prelude::*;
use crate::components::top_menu::TopMenu;
use crate::shared_messages::SharedMessage;
use crate::model::Model;

// For browser debugging
use web_sys::{console, HtmlAnchorElement};



enum AppStates {
    Roster,
    Units,
    // Other states as needed
}

pub struct App {
    // State Machine
    state: AppStates,

    // The Model
    model: Model
}



impl Component for App {
    type Message = SharedMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App { 
            state: AppStates::Units, // Default state
            model: Model::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : SharedMessage) -> bool {
        match msg {

            SharedMessage::ViewRoster => {
                self.state = AppStates::Roster;
                true
            },

            SharedMessage::ViewRoster => {
                self.state = AppStates::Units;
                true
            },

            SharedMessage::Save => {
                // match self.model.to_json() {
                //     Ok(json_string) => {
                //         let document = web_sys::window().unwrap().document().unwrap();
                //         let a: HtmlAnchorElement = document.create_element("a")
                //             .unwrap()
                //             .dyn_into::<HtmlAnchorElement>()
                //             .unwrap();
                        
                //         // Convert the JSON string to a Blob
                //         let mut blob_parts: web_sys::BlobPropertyBag = web_sys::BlobPropertyBag::new();
                //         blob_parts.type_("application/json");
                //         let blob = web_sys::Blob::new_with_str_sequence_and_options(&js_sys::Array::of1(&json_string.into()), &blob_parts).unwrap();
                        
                //         // Create an Object URL from the Blob
                //         let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                        
                //         a.set_href(&url);
                //         a.set_download("roster.json");
                //         a.set_attribute("style", "display: none").unwrap();
                //         document.body().unwrap().append_child(&a).unwrap();
                //         a.click();
                //         a.remove();
                        
                //         // Clean up the Object URL to free resources
                //         web_sys::Url::revoke_object_url(&url).unwrap();
                //     },
                //     Err(e) => {
                //         console::log_1(&format!("Error serializing roster: {:?}", e).into());
                //     }
                // }
                false
            },

            SharedMessage::Load => {
                true
            },

            _ => false,
            // Handle other messages
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <TopMenu 
                    on_switch_to_roster = {ctx.link().callback(|_| SharedMessage::ViewRoster)} 
                    on_switch_to_units = {ctx.link().callback(|_| SharedMessage::ViewUnits)} 
                    on_save = {ctx.link().callback(|_| SharedMessage::Save)} 
                    on_load = {ctx.link().callback(|_| SharedMessage::Load)} 
                />
                {
                    match self.state {
                        AppStates::Roster => html! { /* Roster view component */ },
                        AppStates::Units => html! { /* Units view component */ },
                    }
                }
            </>
        }
    }
}