use fsd28_lib::models::action::Action;
use fsd28_lib::models::class::Tier;
use yew::prelude::*;
use fsd28_lib::models::{
    characteristics::Characteristics, 
    profile::Profile, 
    damage_chart::DamageChart,
    damage_chart::Color,};
use fsd28_lib::get_classes;
use fsd28_lib::get_weapons;
use fsd28_lib::ClassesConfig;
use fsd28_lib::WeaponsConfig;
use crate::components::modal::Modal;
use crate::components::action_tree_view::ActionTreeView;

// For browser debugging
use web_sys::console;


#[derive(Properties, PartialEq, Clone)]
pub struct UnitsViewProps {
    pub profiles: Vec<Profile>, // Assuming Profile is a struct representing your profiles
    pub on_profiles_changed: Callback<Vec<Profile>>,
}

pub struct UnitsView {
    selected_profile: Option<Profile>,
    editing_profile: Option<Profile>,
    show_modal: bool,
}

pub enum Msg {
    ProfileSelected(Profile),
    CreateNewProfile,
    DeleteSelectedProfile,

    // Modal popup for new profile
    ModalConfirm(String),
    ModalCancel,

    // Profile Manipulation
    UpdateFormName(String),
    ProfileEdited,
    SaveProfileChanges,

    // Actions selection
    ActionSelected(Action),
}

impl Component for UnitsView {
    type Message = Msg;
    type Properties = UnitsViewProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            selected_profile: None,
            editing_profile: None,

            show_modal: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProfileSelected(profile) => {
                self.editing_profile = Some(profile.clone());
                self.selected_profile = Some(profile);
                true 
            }

            Msg::CreateNewProfile => {
                // Instead of creating a new profile directly, set show_modal to true

                // TODO set the modal to load different content depending on the request.
                // different lists could be:
                // Class Choice
                // SubClass Choice (With checkbox)
                // Equipment Choice
                // Actions Choice (it's thee actions, should be movable up and down)
                // I could do a different object for each OR handle the Modal object 
                // In a more reusable way. 
                self.show_modal = true;
                true
            },

            Msg::DeleteSelectedProfile => {

                self.update_model_profiles(ctx);
                true
            },

            Msg::UpdateFormName(new_name) => {
                console::log_1(&format!("Name set as {}", new_name.clone()).into());

                if let Some(ref mut profile) = self.editing_profile {
                    profile.name = new_name;
                    
                    ctx.link().send_message(Msg::ProfileEdited);
                }

                console::log_1(&format!("debug1 {:?}", self.editing_profile.as_ref().unwrap().name).into());

                true
            },

            Msg::ProfileEdited => {

                if let Some(updated_profile) = self.editing_profile.as_ref(){

                    self.view_profile(updated_profile);
                }
                true
            },

            Msg::SaveProfileChanges => {

                console::log_1(&format!("Called SAVE").into());

                // Input check: if there's no editing profile doing nothing.
                if self.editing_profile.is_none(){
                    return true
                }
                
                // Signal to update the central view with the edited profile
                let mut all_profiles = ctx.props().profiles.clone();
                console::log_1(&format!("Loaded {} profiles", all_profiles.len()).into());
                console::log_1(&format!("Selected is {}", self.selected_profile.clone().unwrap().name).into());


                // Find the index of the profile to replace
                // TODO remove that double clone(), it's ugly.
                if let Some(index) = all_profiles
                    .iter()
                    .position(|p| p.name == self.selected_profile.clone().unwrap().name) {

                    // Replace the old profile with the updated one
                    all_profiles[index] = self.editing_profile.clone().unwrap().clone();
                }

                ctx.props().on_profiles_changed.emit(all_profiles);
                true
            }

            Msg::ActionSelected(action) => {

                // TODO implement properly
                console::log_1(&format!("selected action is {}", action.name.clone()).into());
                if let Some(ref mut profile) = self.editing_profile {

                    // Check if the action can be added
                    // TODO other checks that must be done are
                    // - if the action is taken already
                    // - if at least one of the "main" actions is selected (if at least one exists)
                    // - if the AD range makes it impossible to use that action.
                    if profile.actions.len() >= 3 {
                        console::log_1(&format!("The profile cannot have more than 3 actions.").into());
                        return true;
                    }

                    profile.actions.push(action);
                    ctx.link().send_message(Msg::ProfileEdited);
                }
                true
            }

            // MODAL VIEW MESSAGES
            Msg::ModalConfirm(class_name) => {
                let classes: ClassesConfig = get_classes("");
                let selected_class: Option<&fsd28_lib::Class> = classes.classes.iter().find(|c| c.name == class_name);
            
                match selected_class {
                    Some(class) => {
                        let mut updated_profiles = ctx.props().profiles.clone();
                        let new_profile = Profile::new("NEW_PROFILE".to_string(), class.clone());
                        updated_profiles.push(new_profile.clone());
                        self.selected_profile = Some(new_profile); // Set the new profile as selected
                        self.editing_profile = self.selected_profile.clone();
                        ctx.props().on_profiles_changed.emit(updated_profiles);
                        self.show_modal = false;
                        true
                    },
                    None => {
                        false
                    }
                }
            },

            Msg::ModalCancel => {
                self.show_modal = false;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let all_classes : Vec<String> = get_classes("").classes.iter().map(|class| class.name.clone()).collect();

        html! {
            <div class="units-view">
                <div class="left-bar">
                    <div class="profiles-list">
                        { for ctx.props().profiles.iter().map(|profile| self.view_profile_button(profile, ctx.link())) }
                    </div>
                    <div class="profile-actions">
                        <button onclick={ctx.link().callback(|_| Msg::CreateNewProfile)}>{"Create New"}</button>
                        <button onclick={ctx.link().callback(|_| Msg::DeleteSelectedProfile)}>{"Delete Selected"}</button>
                    </div>
                </div>
                <div class="center-bar">
                    { self.view_current_profile() }
                </div>
                <div class="right-bar">
                    { self.view_edit_form(ctx) }
                </div>

                {if self.show_modal {
                    html! {
                        <div class="modal">
                            <Modal
                                    classes={all_classes} // Implement this method to get class names
                                    on_confirm={ctx.link().callback(Msg::ModalConfirm)}
                                    on_cancel={ctx.link().callback(|_| Msg::ModalCancel)}
                                />
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }

    // ... other methods ...
}


impl UnitsView {
    fn view_profile_button(&self, profile: &Profile, link: &yew::html::Scope<Self>) -> Html {
        let is_selected = self.selected_profile.as_ref().map_or(false, |p| p == profile);
        let local_profile = profile.clone(); // There is a _DOUBLE_ clone here - TODO FIX this is horrible (but it works)
        html! {
            <button
                class={classes!("left-bar.button", is_selected.then(|| ".selected"))} // SELECTED IS NOT WORKING!
                onclick={link.callback(move |_| Msg::ProfileSelected(local_profile.clone()))}
            >
                { &profile.name }
            </button>
        }
    }

    fn view_profile(&self, profile: &Profile) -> Html {
        console::log_1(&format!("Displaying {}", profile.name).into());
        html! {
            <div class="profile-details">
                <div class="profile-name">{ &profile.name }</div>
                <div class="profile-description">{ &profile.description }</div>
                { self.display_characteristics(&profile.characteristics) }
                { self.display_actions(&profile.actions, &profile.tier) }
                <div class="profile-special-abilities">
                    { "Special Abilities: " }
                    { &profile.special_abilities }
                </div>
                { self.view_damage_chart(&profile.damage_chart) }
            </div>
        }
    }

    fn view_current_profile(&self) -> Html {
        if let Some(profile) = &self.editing_profile {
            self.view_profile(profile)
        } else {
            html! { <div>{ "No profile selected" }</div> }
        }
    }


    fn view_edit_form(&self, ctx: &Context<Self>) -> Html {
        let weapons_config: WeaponsConfig = get_weapons(""); // Load your weapons configuration
    
        html! {
            <div class="edit-form">
                <div class="form-group">
                    <label class="label" for="name">{"NAME:"}</label>
                    <input type="text" id="name"
                        value={self.editing_profile.as_ref().map_or(String::new(), |p| p.name.clone())}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateFormName(input.value())
                        })} />
                    <ActionTreeView 
                        weapons={weapons_config.weapons} 
                        selected_actions={vec![]} 
                        on_action_select={ctx.link().callback(move |action: Action| Msg::ActionSelected(action))}
                    />
                </div>
                <button onclick={ctx.link().callback(|_| Msg::SaveProfileChanges)}>
                    {"Save Changes"}
                </button>
            </div>
        }
    }

    fn display_characteristics (&self, characteristics: &Characteristics) -> Html {
        html! {
            <div class="profile-stats">
                <div class="profile-stats-cmd">{ format!("Cmd: {}", characteristics.stat_cmd) }</div>
                <div class="profile-stats-def">{ format!("Def: {}", characteristics.stat_def) }</div>
                <div class="profile-stats-save">{ format!("Save: {}", characteristics.stat_save.display()) }</div>
                <div class="profile-stats-move">{ format!("Move: {}", characteristics.stat_move) }</div>
                <div class="profile-stats-shoot">{ format!("Shoot: {}", characteristics.stat_shoot.display()) }</div>
                <div class="profile-stats-melee">{ format!("Melee: {}", characteristics.stat_melee.display()) }</div>
            </div>
        }
    }

    fn display_actions (&self, actions: &Vec<Action>, tier: &Tier) -> Html {
        html! {
            {for actions.iter().enumerate().map(|(index, action)| {
                html! { <div class="action-text">{format!("S{}\n {}", index + 1, action.display_ascii(tier))}</div> }
            })}
        }
    }

    fn view_damage_chart(&self, damage_chart: &DamageChart) -> Html {
        html! {
            <div class="damage-chart">
                { self.view_damage_chart_top_row() }
                { self.view_damage_chart_bottom_row(&damage_chart) }
            </div>
        }
    }

    fn view_damage_chart_top_row(&self) -> Html {
        html! {
            <>
            { for (1..=6).map(|num| self.view_damage_chart_top_cell(num, 1, 1)) }
            </>
        }
    }
    
    fn view_damage_chart_top_cell(&self, content: i32, row: i32, col_span: i32) -> Html {
        let style = format!("grid-row: {}; grid-column: {} / span {};", row, content, col_span);
        html! {
            <div {style}>
                { content }
            </div>
        }
    }

    fn view_damage_chart_bottom_row(&self, damage_chart: &DamageChart) -> Html {
        let mut current_column = 1;
        html! {
            <>
                { for damage_chart.intervals.iter().map(|interval| {
                    let (span, color, content) = interval;
                    let cell = self.view_damage_chart_bottom_cell(current_column, *span, color, content);
                    current_column += span;
                    cell
                }) }
            </>
        }
    }
    
    fn view_damage_chart_bottom_cell(&self, start: u32, span: u32, color: &Color, content: &str) -> Html {
        let style = format!(
            "grid-row: 2; grid-column: {} / span {}; background-color: {}; border-right: {};", 
            start, span, self.color_to_css(color), if start + span < 7 { "1px solid black" } else { "none" }
        );
    
        html! {
            <div {style}>
                { content }
            </div>
        }
    }

    fn color_to_css(&self, color: &Color) -> &'static str {
        match color {
            Color::Red => "red",
            Color::Yellow => "yellow",
            Color::Green => "green",
            // Add other colors as needed
        }
    }

    fn update_model_profiles(&mut self, ctx: &Context<Self>) {
        ctx.props().on_profiles_changed.emit(ctx.props().profiles.clone());
    }
}
