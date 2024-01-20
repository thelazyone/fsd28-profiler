use yew::prelude::*;
use crate::components::top_menu::TopMenu;
// Import other components as needed

enum AppState {
    Roster,
    Units,
    // Other states as needed
}

struct Model {
    link: ComponentLink<Self>,
    state: AppState,
}

enum Msg {
    SwitchToRoster,
    SwitchToUnits,
    // Other messages as needed
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link,
            state: AppState::Roster, // Default state
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchToRoster => {
                self.state = AppState::Roster;
                true
            },
            Msg::SwitchToUnits => {
                self.state = AppState::Units;
                true
            },
            // Handle other messages
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <TopMenu 
                    on_switch_to_roster=self.link.callback(|_| Msg::SwitchToRoster)
                    on_switch_to_units=self.link.callback(|_| Msg::SwitchToUnits)
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

fn main() {
    yew::start_app::<Model>();
}
