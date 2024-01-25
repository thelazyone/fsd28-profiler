use yew::prelude::*;

pub struct UnitsView;

impl Component for UnitsView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="units-view">
                <div class="left-bar">{ "List of Profiles" }</div>
                <div class="right-area">{ "Profile Viewer" }</div>
            </div>
        }
    }
}