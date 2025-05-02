use yew::prelude::*;
use fsd28_lib::models::profile::Profile;
use super::card_generator::CardGenerator;

#[derive(Properties, PartialEq)]
pub struct RosterViewProps {
    pub profiles: Vec<Profile>,
    pub on_profiles_changed: Callback<Vec<Profile>>,
}

pub struct RosterView {
    selected_profiles: Vec<Profile>,
}

pub enum Msg {
    ToggleProfile(Profile),
}

impl Component for RosterView {
    type Message = Msg;
    type Properties = RosterViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_profiles: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleProfile(profile) => {
                if let Some(pos) = self.selected_profiles.iter().position(|p| p.name == profile.name) {
                    self.selected_profiles.remove(pos);
                } else {
                    self.selected_profiles.push(profile);
                }
                ctx.props().on_profiles_changed.emit(self.selected_profiles.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total_points = self.selected_profiles.iter()
            .fold(0u32, |acc, profile| acc + profile.cost);

        let selected_profile = self.selected_profiles.first().cloned();

        html! {
            <div class="roster-view">
                <div class="unit-list">
                    { for ctx.props().profiles.iter().map(|profile| self.view_profile_button(ctx, profile)) }
                </div>
                <div class="selected-units">
                    if let Some(profile) = selected_profile {
                        <CardGenerator profile={profile} />
                    } else {
                        <div class="no-selection">
                            { "Select a unit to view its card" }
                        </div>
                    }
                    <div class="total-points">
                        { format!("Total Points: {}", total_points) }
                    </div>
                </div>
            </div>
        }
    }
}

impl RosterView {
    fn view_profile_button(&self, ctx: &Context<Self>, profile: &Profile) -> Html {
        let is_selected = self.selected_profiles.iter().any(|p| p.name == profile.name);
        let button_text = format!("{} ({})", profile.name, profile.cost);
        let onclick_profile = profile.clone();
        
        html! {
            <button
                class={classes!("button", if is_selected { "selected" } else { "" })}
                onclick={ctx.link().callback(move |_| Msg::ToggleProfile(onclick_profile.clone()))}
            >
                { button_text }
            </button>
        }
    }
}
