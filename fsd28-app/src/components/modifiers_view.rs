use yew::prelude::*;
use fsd28_lib::Modifier; 

#[derive(Properties, PartialEq, Clone)]
pub struct ModifiersViewProps {
    #[prop_or_default]
    pub modifiers: Vec<Modifier>, // List of all available modifiers for the selected class
    #[prop_or_default]
    pub selected_modifiers: Vec<Modifier>, // IDs of currently selected modifiers
    #[prop_or_default]
    pub on_modifier_toggle: Callback<Modifier>, // Callback to handle toggling modifiers
    #[prop_or_default]
    pub available_classes: Vec<String>, // List of all available classes
    #[prop_or_default]
    pub current_class: String, // Current selected class
    #[prop_or_default]
    pub on_class_change: Callback<String>, // Callback to handle class changes
}

#[derive(Clone)]
pub enum Msg {
    ToggleModifier(Modifier), // Pass the ID of the modifier to toggle
    ClassSelected(String), // Handle class selection
    ToggleDropdown, // Toggle the dropdown menu
}

pub struct ModifiersView {
    is_class_dropdown_open: bool,
}

impl Component for ModifiersView {
    type Message = Msg;
    type Properties = ModifiersViewProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            is_class_dropdown_open: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleModifier(modifier) => {
                ctx.props().on_modifier_toggle.emit(modifier);
                true // Re-render as needed
            },
            Msg::ClassSelected(class_name) => {
                ctx.props().on_class_change.emit(class_name);
                self.is_class_dropdown_open = false;
                true
            },
            Msg::ToggleDropdown => {
                self.is_class_dropdown_open = !self.is_class_dropdown_open;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dropdown_symbol = if self.is_class_dropdown_open { "▼" } else { "▲" };
        let current_class = ctx.props().current_class.clone();

        html! {
            <div class="right-bar-section">
                <div class="section-header">
                    <span>{"CLASS AND MODIFIERS"}</span>
                </div>
                <hr/>
                <div class="atw-weapon">
                    <div class="atw-weapon-name" onclick={ctx.link().callback(|_| Msg::ToggleDropdown)}>
                        { format!("Class: {} {}", &current_class, dropdown_symbol) }
                    </div>
                    if self.is_class_dropdown_open {
                        <div class="atw-weapon-options">
                            { for ctx.props().available_classes.iter().map(|class_name| {
                                let class_name_clone = class_name.clone();
                                let is_selected = class_name == &current_class;
                                html! {
                                    <div class={classes!("atw-action", is_selected.then_some("atw-action-selected"))} 
                                         onclick={ctx.link().callback(move |_| Msg::ClassSelected(class_name_clone.clone()))}>
                                        { class_name }
                                    </div>
                                }
                            })}
                        </div>
                    }
                </div>
                <hr/>
                { for ctx.props().modifiers.iter().map(|modifier| self.view_modifier(modifier, ctx)) }
            </div>
        }
    }
}

impl ModifiersView {
    fn view_modifier(&self, modifier: &Modifier, ctx: &Context<Self>) -> Html {
        let is_selected = ctx.props().selected_modifiers.contains(modifier);
        let local_modifier = modifier.clone();
        html! {
            <div class="modifier">
                <label class="custom-checkbox">
                    { format!("{} ({})", &local_modifier.id, &local_modifier.points) }
                    <input type="checkbox"
                        checked={is_selected}
                        onclick={ctx.link().callback(move |_| Msg::ToggleModifier(local_modifier.clone()))} />
                    <span class="checkmark"></span> // Custom checkmark
                </label>
            </div>
        }
    }
}