use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub on_switch_to_roster: Callback<()>,
    pub on_switch_to_units: Callback<()>,
    // Other props as needed
}

pub struct TopMenu {
    props: Props,
}

impl Component for TopMenu {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="top-menu">
                <button onclick=self.props.on_switch_to_roster.reform(|_| ())>{ "Roster" }</button>
                <button onclick=self.props.on_switch_to_units.reform(|_| ())>{ "Units" }</button>
                // Other buttons
            </div>
        }
    }
}
