use yew::prelude::*;
use fsd28_lib::models::profile::Profile;

#[derive(Properties, PartialEq, Clone)]
pub struct RosterViewProps {
    pub profiles: Vec<Profile>,
    pub on_profiles_changed: Callback<Vec<Profile>>,
}

pub struct RosterView {
    selected_profiles: Vec<Profile>,
}

pub enum Msg {
    ProfileToggled(Profile),
}

impl Component for RosterView {
    type Message = Msg;
    type Properties = RosterViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_profiles: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProfileToggled(profile) => {
                if let Some(pos) = self.selected_profiles.iter().position(|p| p.name == profile.name) {
                    self.selected_profiles.remove(pos);
                } else {
                    self.selected_profiles.push(profile);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total_points: u32 = self.selected_profiles.iter()
            .map(|p| p.get_final_profile().cost)
            .fold(0u32, |acc, x| acc + x);

        html! {
            <div class="roster-view">
                <div class="left-bar">
                    <div class="profiles-list">
                        { for ctx.props().profiles.iter().map(|profile| self.view_profile_button(profile, ctx.link())) }
                    </div>
                </div>
                <div class="center-bar">
                    <div class="selected-units">
                        <h3>{ "Selected Units" }</h3>
                        { for self.selected_profiles.iter().map(|profile| {
                            let final_profile = profile.get_final_profile();
                            html! {
                                <div class="selected-unit">
                                    { format!("{} ({} points)", final_profile.name, final_profile.cost) }
                                </div>
                            }
                        })}
                        <div class="total-points">
                            { format!("Total Points: {}", total_points) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl RosterView {
    fn view_profile_button(&self, profile: &Profile, link: &yew::html::Scope<Self>) -> Html {
        let is_selected = self.selected_profiles.iter().any(|p| p.name == profile.name);
        let final_profile = profile.get_final_profile();
        let local_profile = profile.clone();
        
        html! {
            <button
                class={classes!("button", is_selected.then_some("selected"))}
                onclick={link.callback(move |_| Msg::ProfileToggled(local_profile.clone()))}
            >
                { format!("{} ({} points)", final_profile.name, final_profile.cost) }
            </button>
        }
    }
}
