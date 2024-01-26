use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub classes: Vec<String>,
    pub on_confirm: Callback<String>,
    pub on_cancel: Callback<()>,
}

pub struct Modal;
pub enum ModalMsg {
    Confirm(String),
    Cancel,
}

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        Modal // Just return Modal here
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMsg::Confirm(class_name) => {
                ctx.props().on_confirm.emit(class_name);
                false // No need to rerender
            },
            ModalMsg::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="modal">
                <div class="modal-content">
                    <h2>{"Select a class"}</h2>
                    <ul>
                        { for ctx.props().classes.iter().map(|class_name| {
                            let class_name_clone = class_name.clone();
                            html! {
                                <li onclick={ctx.link().callback(move |_| ModalMsg::Confirm(class_name_clone.clone()))}>
                                    { class_name }
                                </li>
                            }
                        })}
                    </ul>
                    <button onclick={ctx.link().callback(|_| ModalMsg::Cancel)}>{"Cancel"}</button>
                </div>
            </div>
        }
    }
}
