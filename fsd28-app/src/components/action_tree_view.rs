use yew::prelude::*;
use crate::models::{Weapon, Action}; // Adjust paths as necessary

#[derive(Properties, PartialEq, Clone)]
pub struct ActionTreeViewProps {
    pub weapons: Vec<Weapon>,
    pub selected_actions: Vec<String>, // Track selected actions by name
    pub on_action_select: Callback<Action>,
    // Additional props as needed to determine action availability
}

pub struct ActionTreeView {
    // Local state can be added if needed
}

pub enum Msg {
    ToggleAction(String), // Toggle the selection state of an action
}

impl Component for ActionTreeView {
    type Message = Msg;
    type Properties = ActionTreeViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { /* Initialize state here if needed */ }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleAction(action_name) => {
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
                { for ctx.props().weapons.iter().map(|weapon| self.view_weapon(weapon, ctx)) }
            </div>
        }
    }
}

impl ActionTreeView {
    fn view_weapon(&self, weapon: &Weapon, ctx: &Context<Self>) -> Html {
        html! {
            <div class="weapon">
                <div class="weapon-name">{ &weapon.name }</div>
                <div class="weapon-options">
                    { for weapon.options.iter().map(|option| self.view_option(option, ctx)) }
                </div>
            </div>
        }
    }

    fn view_option(&self, option: &WeaponOption, ctx: &Context<Self>) -> Html {
        let is_selected = ctx.props().selected_actions.contains(&option.action.name);
        // Placeholder: Determine if the action is available based on your game's logic
        let is_available = true; // This should be dynamically calculated
        let class = match (is_selected, is_available) {
            (true, _) => "action selected",
            (_, false) => "action unavailable",
            _ => "action",
        };

        html! {
            <div
                class={class}
                onclick={ctx.link().callback(move |_| Msg::ToggleAction(option.action.name.clone()))}
            >
                { &option.action.name }
            </div>
        }
    }
}
