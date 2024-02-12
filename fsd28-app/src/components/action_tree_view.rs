use yew::prelude::*;
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
            <div class="right-bar-section">
                { html!{"ACTIONS SELECTION"} }
                <hr/>
                { for ctx.props().weapons.iter().map(|weapon| self.view_weapon(weapon, ctx)) }
            </div>
        }
    }
}

// TODO the already selected actoins should be shown as grey
impl ActionTreeView {
    fn view_weapon(&self, weapon: &Weapon, ctx: &Context<Self>) -> Html {
        let weapon_name = weapon.name.clone();
        let is_expanded = self.expanded_weapon.as_ref() == Some(&weapon.name);
        let toggle_msg = Msg::ToggleWeapon(weapon_name.clone());
        let expansion_symbol = if is_expanded { "▼" } else { "▲" };

        html! {
            <div class="atw-weapon">
                <div class="atw-weapon-name" onclick={ctx.link().callback(move |_| toggle_msg.clone())}>
                    { format!("{} {}", &weapon_name, expansion_symbol) }
                </div>
                if is_expanded {
                    <div class="atw-weapon-options">
                        { for weapon.options.iter().map(|option| self.view_option(option, &weapon.options, ctx)) }
                    </div>
                }
            </div>
        }
    }

    fn view_option(&self, option: &WeaponOption, all_options: &[WeaponOption], ctx: &Context<Self>) -> Html {
        let action_name = option.action.name.clone();
        let is_selected = ctx.props().selected_actions.contains(&action_name);

        // To check if the action is available the only way is to check one of the following 
        // - there is no mandatory action
        // - it is a mandatory action
        // - At least a mandatory action is selected
        let is_there_base_option = all_options.iter().any(|opt| {opt.is_base});
        let is_base_option_selected = all_options.iter().any(|opt| {
            opt.is_base && ctx.props().selected_actions.contains(&opt.action.name)
        });
        let is_too_many_actions = ctx.props().selected_actions.len() >= 3; // TODO This should not be a magic number

        let is_available = !is_too_many_actions && (!is_there_base_option || option.is_base || is_base_option_selected);

        let class_string = match (is_selected, is_available) {
            (true, _) => "atw-action-selected",
            (_, false) => "atw-action-unavailable",
            _ => "",
        };


        html! {
            <div class="atw-action">
                {
                    if !class_string.is_empty() {
                        html! { <div class={class_string}> 
                            { format!("{} ({})", &option.action.name, &option.action.points) } 
                        </div> }
                    }
                    else {
                        html! {
                            <div onclick={ctx.link().callback(move |_| Msg::SelectAction(action_name.clone()))}>
                                { format!("{} ({})", &option.action.name, &option.action.points) }
                            </div>
                        }
                    }
                }
            </div>
        }
    }
}
