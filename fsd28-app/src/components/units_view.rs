use yew::prelude::*;
use fsd28_lib::models::{characteristics::{self, Characteristics}, profile::Profile, damage_chart::DamageChart, damage_chart::Color};
use fsd28_lib::get_classes;
use fsd28_lib::ClassesConfig;

use crate::shared_messages::SharedMessage;
use crate::components::modal::Modal;


#[derive(Properties, PartialEq, Clone)]
pub struct UnitsViewProps {
    pub profiles: Vec<Profile>, // Assuming Profile is a struct representing your profiles
    pub on_profiles_changed: Callback<Vec<Profile>>,
}

pub struct UnitsView {
    selected_profile: Option<Profile>,
    show_modal: bool,
}

pub enum Msg {
    ProfileSelected(Profile),
    CreateNewProfile,
    DeleteSelectedProfile,
    UpdateName(String),

    // Modal popup for new profile
    ShowModal,
    ModalConfirm(String),
    ModalCancel,
}

impl Component for UnitsView {
    type Message = Msg;
    type Properties = UnitsViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected_profile: None,
            show_modal: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProfileSelected(profile) => {
                self.selected_profile = Some(profile);
                true 
            }

            Msg::CreateNewProfile => {
                // Instead of creating a new profile directly, set show_modal to true
                self.show_modal = true;
                true
            },

            Msg::DeleteSelectedProfile => {

                self.update_model_profiles(ctx);
                true
            },

            Msg::UpdateName(new_name) => {
                // Update the name of the selected profile
                if let Some(profile) = &mut self.selected_profile {
                    profile.name = new_name;
                }
                true
            },

            // MODAL VIEW MESSAGES
            Msg::ShowModal => {
                self.show_modal = true;
                true
            },

            Msg::ModalConfirm(class_name) => {
                let classes: ClassesConfig = get_classes("");
                let selected_class = classes.classes.iter().find(|c| c.name == class_name);
            
                match selected_class {
                    Some(class) => {
                        let mut updated_profiles = ctx.props().profiles.clone();
                        let new_profile = Profile::new("NEW_PROFILE".to_string(), class.clone());
                        updated_profiles.push(new_profile);
                        ctx.props().on_profiles_changed.emit(updated_profiles);
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
                        { for ctx.props().profiles.iter().map(|profile| self.view_profile(profile, ctx.link())) }
                    </div>
                    <div class="profile-actions">
                        <button onclick={ctx.link().callback(|_| Msg::CreateNewProfile)}>{"Create New"}</button>
                        <button onclick={ctx.link().callback(|_| Msg::DeleteSelectedProfile)}>{"Delete Selected"}</button>
                    </div>
                    <div class="center-bar">
                            { self.view_selected_profile() }
                    </div>
                    <div class="right-bar">
                        //{ self.view_edit_name_form(ctx) }
                    </div>
                </div>

                {if self.show_modal {
                    html! {
                        <Modal
                            classes={all_classes} // Implement this method to get class names
                            on_confirm={ctx.link().callback(Msg::ModalConfirm)}
                            on_cancel={ctx.link().callback(|_| Msg::ModalCancel)}
                        />
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
    fn view_profile(&self, profile: &Profile, link: &yew::html::Scope<Self>) -> Html {
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

    fn view_selected_profile(&self) -> Html {
        if let Some(profile) = &self.selected_profile {
            // let ascii_content = profile.display_ascii();
            // let ascii_html = ascii_content.split('\n').map(|line| html! { <>{line}<br/></> }).collect::<Html>();
            // html! { <div>{ ascii_html }</div> }

            html! {
                <div class="profile-details">
                    <div class="profile-name">{ &profile.name }</div>
                    <div class="profile-description">{ &profile.description }</div>
                    { self.display_characteristics(&profile.characteristics) }
                    <div class="profile-special-abilities">
                        { "Special Abilities: " }
                        { &profile.special_abilities }
                    </div>
                    { self.view_damage_chart(&profile.damage_chart) }
                </div>
            }

        } else {
            html! { <div>{ "No profile selected" }</div> }
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
