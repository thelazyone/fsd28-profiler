use yew::prelude::*;
use fsd28_lib::Modifier; // Adjust the path as necessary

#[derive(Properties, PartialEq, Clone)]
pub struct ModifiersViewProps {
    pub modifiers: Vec<Modifier>, // List of all available modifiers for the selected class
    pub selected_modifiers: Vec<String>, // IDs of currently selected modifiers
    pub on_modifier_toggle: Callback<String>, // Callback to handle toggling modifiers
}

#[derive(Clone)]
pub enum Msg {
    ToggleModifier(String), // Pass the ID of the modifier to toggle
}

pub struct ModifiersView {
    // Local state can be defined here if needed
}

impl Component for ModifiersView {
    type Message = Msg;
    type Properties = ModifiersViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            // Initialize state here if necessary
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleModifier(id) => {
                ctx.props().on_modifier_toggle.emit(id);
                true // Re-render as needed
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="right-bar-section">
                { html!{"CLASS MODIFIERS"} }
                    <hr/>
                { for ctx.props().modifiers.iter().map(|modifier| self.view_modifier(modifier, ctx)) }
            </div>
        }
    }

}

impl ModifiersView {
    fn view_modifier(&self, modifier: &Modifier, ctx: &Context<Self>) -> Html {
        let is_selected = ctx.props().selected_modifiers.contains(&modifier.id);
        let label = modifier.id.clone();
        html! {
            <div class="modifier">
                <label>
                    <input type="checkbox"
                        checked={is_selected}
                        onclick={ctx.link().callback(move |_| Msg::ToggleModifier(label.clone()))} />
                    { &modifier.id }
                </label>
            </div>
        }
    }
}