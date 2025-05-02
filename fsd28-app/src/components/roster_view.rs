use yew::prelude::*;
use fsd28_lib::models::profile::Profile;

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
    ExportList,
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
            Msg::ToggleProfile(profile) => {
                if let Some(pos) = self.selected_profiles.iter().position(|p| p.name == profile.name) {
                    self.selected_profiles.remove(pos);
                } else {
                    self.selected_profiles.push(profile);
                }
                true
            }
            Msg::ExportList => {
                // TODO: Implement export functionality
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total_points: u32 = self.selected_profiles.iter()
            .map(|p| p.get_final_profile().cost)
            .sum();

        html! {
            <div class="roster-view">
                <div class="left-bar">
                    <div class="profiles-list">
                        { for ctx.props().profiles.iter().map(|profile| self.view_profile_button(profile, ctx.link())) }
                    </div>
                </div>
                <div class="center-bar">
                    <div class="total-points">
                        { format!("Total Points: {}", total_points) }
                    </div>
                </div>
                <div class="right-bar">
                    <button onclick={ctx.link().callback(|_| Msg::ExportList)}>
                        { "Export List" }
                    </button>
                </div>
            </div>
        }
    }
}

impl RosterView {
    fn view_profile_button(&self, profile: &Profile, link: &yew::html::Scope<Self>) -> Html {
        let is_selected = self.selected_profiles.iter().any(|p| p.name == profile.name);
        let profile_clone = profile.clone();
        let final_profile = profile.get_final_profile();
        html! {
            <button
                class={classes!("button", is_selected.then_some("selected"))}
                onclick={link.callback(move |_| Msg::ToggleProfile(profile_clone.clone()))} >
                { format!("{} ({} points)", &profile.name, &final_profile.cost) }
            </button>
        }
    }
}
