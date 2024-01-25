use yew::prelude::*;
use fsd28_lib::models::profile::Profile;

#[derive(Properties, PartialEq, Clone)]
pub struct UnitsViewProps {
    pub profiles: Vec<Profile>, // Assuming Profile is a struct representing your profiles
}

pub struct UnitsView {
    selected_profile: Option<Profile>,
}

pub enum Msg {
    ProfileSelected(Profile),
}

impl Component for UnitsView {
    type Message = Msg;
    type Properties = UnitsViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected_profile: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProfileSelected(profile) => {
                self.selected_profile = Some(profile);
                true // Rerender the component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="units-view">
                <div class="left-bar">
                    { for ctx.props().profiles.iter().map(|profile| self.view_profile(profile, ctx.link())) }
                </div>
                <div class="center-bar">
                    { self.view_selected_profile() }
                </div>
                <div class="right-bar">
                    // Content for the right bar
                </div>
            </div>
        }
    }

    // ... other methods ...
}


impl UnitsView {
    // ... other methods ...

    fn view_profile(&self, profile: &Profile, link: &yew::html::Scope<Self>) -> Html {
        let is_selected = self.selected_profile.as_ref().map_or(false, |p| p == profile);
        let local_profile = profile.clone(); // There is a _DOUBLE_ clone here - TODO FIX this is horrible (but it works)
        html! {
            <button
                class={classes!("profile-button", is_selected.then(|| "selected"))}
                onclick={link.callback(move |_| Msg::ProfileSelected(local_profile.clone()))}
            >
                { &profile.name }
            </button>
        }
    }

    fn view_selected_profile(&self) -> Html {
        if let Some(profile) = &self.selected_profile {
            html! { <div>{ &profile.display_ascii() }</div> }
        } else {
            html! { <div>{ "No profile selected" }</div> }
        }
    }
}
