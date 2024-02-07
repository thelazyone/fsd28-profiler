use yew::prelude::*;
use std::collections::HashSet;
use fsd28_lib::{Weapon, WeaponOption, Action}; 

#[derive(Properties, PartialEq, Clone)]
pub struct ActionTreeViewProps {
    pub weapons: Vec<Weapon>,
    pub selected_actions: Vec<String>, // Track selected actions by name // TODO MAYBE TBR??
    pub on_action_select: Callback<Action>,
}


pub struct ActionTreeView {
    expanded_weapon: Option<String>, // Track the currently expanded weapon by name
}

#[derive(Clone)]
pub enum Msg {
    ToggleWeapon(String), // Toggle the expanded state of a weapon
    SelectAction(String), // Handle action selection
}

impl Component for ActionTreeView {
    type Message = Msg;
    type Properties = ActionTreeViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            expanded_weapon: None, // Starting with all weapons collapsed
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::ToggleWeapon(weapon_name) => {
                if Some(&weapon_name) == self.expanded_weapon.as_ref() {
                    self.expanded_weapon = None; // Collapse if it's already expanded
                } else {
                    self.expanded_weapon = Some(weapon_name); // Expand the clicked weapon
                }
                true // Re-render
            },
            
            Msg::SelectAction(action_name) => {
                let action = ctx.props().weapons.iter()
                    .flat_map(|weapon| &weapon.options)
                    .find(|option| option.action.name == action_name)
                    .map(|option| &option.action);

                if let Some(action) = action {
                    ctx.props().on_action_select.emit(action.clone());
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="action-tree-view">
                { html!{"ACTIONS SELECTION"} }
                <hr/>
                { for ctx.props().weapons.iter().map(|weapon| self.view_weapon(weapon, ctx)) }
            </div>
        }
    }
}

impl ActionTreeView {
    fn view_weapon(&self, weapon: &Weapon, ctx: &Context<Self>) -> Html {
        let weapon_name = weapon.name.clone();
        let is_expanded = self.expanded_weapon.as_ref() == Some(&weapon.name);
        let toggle_msg = Msg::ToggleWeapon(weapon_name.clone());

        html! {

            <div class="atw-weapon">
                <div class="atw-weapon-name" onclick={ctx.link().callback(move |_| toggle_msg.clone())}>
                    { &weapon_name }
                </div>
                if is_expanded {
                    <div class="atw-weapon-options">
                        { for weapon.options.iter().map(|option| self.view_option(option, ctx)) }
                    </div>
                }
            </div>
        }
    }

    fn view_option(&self, option: &WeaponOption, ctx: &Context<Self>) -> Html {
        let action_name = option.action.name.clone();
        let is_selected = ctx.props().selected_actions.contains(&action_name);
        // Placeholder for determining if the action is available or not
        let is_available = true; // This should be dynamically calculated based on your logic

        let class = match (is_selected, is_available) {
            (true, _) => "atw-action selected",
            (_, false) => "atw-action unavailable",
            _ => "atw-action",
        };

        html! {
            <div class={class} onclick={ctx.link().callback(move |_| Msg::SelectAction(action_name.clone()))}>
                { &option.action.name }
            </div>
        }
    }
}
