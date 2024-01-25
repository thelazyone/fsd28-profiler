use yew::prelude::*;

pub struct RosterView;

impl Component for RosterView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "TEMP ROSTER VIEW" }</div>
        }
    }
}