use yew::prelude::*;

use crate::shared_messages::SharedMessage;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub on_switch_to_roster: Callback<SharedMessage>,
    pub on_switch_to_units: Callback<SharedMessage>,
}

pub struct TopMenu {
}

impl Component for TopMenu {
    type Message = SharedMessage;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        TopMenu {
        }    
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {

        // Nothing to do here
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="top-menu">
                <div class="title">
                    <span class="title">{"FSD28 - EXPERIMENTAL ARMY BUILDER"}</span>
                </div>
                <div class="menu">
                    <button onclick={ctx.props().on_switch_to_roster.reform(|_| SharedMessage::ViewRoster)}>{ "View Roster" }</button>
                    <button onclick={ctx.props().on_switch_to_units.reform(|_| SharedMessage::ViewUnits)}>{ "View Units" }</button>
                </div>
            </div>
        }
    }
}
