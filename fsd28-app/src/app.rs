use yew::prelude::*;
use crate::components::top_menu::TopMenu;
// Import other components as needed
use crate::shared_messages::SharedMessage;

enum AppState {
    Roster,
    Units,
    // Other states as needed
}

pub struct App {
    state: AppState,
}


impl Component for App {
    type Message = SharedMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App { 
            state: AppState::Units, // Default state
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : SharedMessage) -> bool {
        match msg {

            SharedMessage::ViewRoster => {
                self.state = AppState::Roster;
                true
            },

            SharedMessage::ViewRoster => {
                self.state = AppState::Units;
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
                    // Pass other callbacks as needed
                />
                {
                    match self.state {
                        AppState::Roster => html! { /* Roster view component */ },
                        AppState::Units => html! { /* Units view component */ },
                        // Render other states
                    }
                }
            </>
        }
    }
}