use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::{
    top_menu::TopMenu,
    roster_view::RosterView,
    units_view::UnitsView,
};
use crate::shared_messages::SharedMessage;
use crate::model::Model;

// For browser debugging
use web_sys::console;

// Handling the loaded stuff as a pointer
// TODO is this even necessary? It's *NOT* idiomatic!
use std::rc::Rc;

enum AppStates {
    Roster,
    Units,
    // Other states as needed
}

pub struct App {
    // State Machine
    state: AppStates,

    // The Model
    model: Model,

    // input file
    file_input_ref: NodeRef,

    // Flags
    reset_selected: bool,
}



impl Component for App {
    type Message = SharedMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App { 
            state: AppStates::Units, // Default state
            model: Model::new(),
            file_input_ref: NodeRef::default(),
            reset_selected: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg : SharedMessage) -> bool {
        match msg {

            SharedMessage::ViewRoster => {
                self.state = AppStates::Roster;
                true
            },

            SharedMessage::ViewUnits => {
                self.state = AppStates::Units;
                true
            },

            // if loaded a file: 
            SharedMessage::FileContentReceived(text) => {
                match Model::from_json(&text) {
                    Ok(model) => {
                        self.reset_selected = true;
                        self.model = model;
                    }

                    Err(_e) => {
                        // Do something TODO
                    }
                }

                true
            }

            SharedMessage::Load => {
                if let Some(input) = self.file_input_ref.cast::<web_sys::HtmlInputElement>() {
                    
                    // This will only trigger the file dialog. Everything else has been set up as a closure
                    // In the create() method.
                    input.click(); 
                }
                true
            }

            SharedMessage::Save => {
                match self.model.to_json() {
                    Ok(json_string) => {
                        let document = web_sys::window().unwrap().document().unwrap();
                        let a = document.create_element("a")
                            .unwrap()
                            .dyn_into::<web_sys::HtmlAnchorElement>()
                            .unwrap();
                        
                        // Convert the JSON string to a Blob
                        let mut blob_parts: web_sys::BlobPropertyBag = web_sys::BlobPropertyBag::new();
                        blob_parts.type_("application/json");
                        let blob = web_sys::Blob::new_with_str_sequence_and_options(&js_sys::Array::of1(&json_string.into()), &blob_parts).unwrap();
                        
                        // Create an Object URL from the Blob
                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                        
                        a.set_href(&url);
                        a.set_download("profiles.json");
                        a.set_attribute("style", "display: none").unwrap();
                        document.body().unwrap().append_child(&a).unwrap();
                        a.click();
                        a.remove();
                        
                        // Clean up the Object URL to free resources
                        web_sys::Url::revoke_object_url(&url).unwrap();
                    },
                    Err(e) => {
                        console::log_1(&format!("Error serializing profiles: {:?}", e).into());
                    }
                }
                false
            },

            SharedMessage::UpdateProfiles(updated_profiles) => {
                self.model.profiles = updated_profiles;
                self.reset_selected = false;
                true
            },

            _ => false,
            // Handle other messages
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // Loading the models necessary for the sub-views.
        let profiles = self.model.profiles.clone();

        html! {
            <div>
                <div class="mobile-warning">
                    { "This app is not designed for mobile (yet)." }
                </div>
                <div class = "app">
                    <TopMenu 
                        on_switch_to_roster = {ctx.link().callback(|_| SharedMessage::ViewRoster)} 
                        on_switch_to_units = {ctx.link().callback(|_| SharedMessage::ViewUnits)} 
                        on_save = {ctx.link().callback(|_| SharedMessage::Save)} 
                        on_load = {ctx.link().callback(|_| SharedMessage::Load)} 
                    />
                    {
                        match self.state {
                            AppStates::Roster => html! { <RosterView /> },
                            AppStates::Units => html! { <UnitsView 
                                profiles={profiles} 
                                on_profiles_changed={ctx.link().callback(SharedMessage::UpdateProfiles)}
                                reset_selected={self.reset_selected}
                                /> },
                        }
                    }

                    // File Selection Popup
                    <input type="file" ref={self.file_input_ref.clone()} style="display: none" onchange={
                        let link_clone_outer = ctx.link().clone(); // Clone the link outside of the callback
                        let link_clone_inner = link_clone_outer.clone(); // Clone the link for the inner closure
                        link_clone_outer.callback(move |event: web_sys::Event| {
                            if let Some(target) = event.target() {
                                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                    if let Some(files) = input.files() {
                                        if let Some(file) = files.get(0) {
                                            
                                            // Get the name of the file
                                            let file_name = file.name();
                                            console::log_1(&format!("Selected file name: {}", file_name).into());
                                
                                            // Read the content of the file
                                            let file_reader = web_sys::FileReader::new().unwrap();
                                            let file_reader_rc = Rc::new(file_reader); // Wrap the FileReader in an Rc
                                            let file_reader_clone = file_reader_rc.clone(); // Clone the Rc for the closure

                                            // Clone the link for the onload closure. Yep, another cloning.
                                            let link_clone_for_onload = link_clone_inner.clone();
                                            let onload_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                                                if let Ok(result) = file_reader_clone.result() {
                                                    if let Some(text) = result.as_string() {
                                                        
                                                        link_clone_for_onload.send_message(SharedMessage::FileContentReceived(text));
                                                    }
                                                }
                                            }) as Box<dyn FnMut(_)>);
                                
                                            file_reader_rc.add_event_listener_with_callback("load", onload_closure.as_ref().unchecked_ref()).unwrap();
                                            onload_closure.forget();
                                
                                            file_reader_rc.read_as_text(&file).unwrap();

                                            // Resetting the input file for hte next time
                                            input.set_value("");
                                        }
                                    }
                                }
                            }
                            SharedMessage::NoOp // Return a dummy message
                        })
                    }/>
                </div>
            </div>
        }
    }
}