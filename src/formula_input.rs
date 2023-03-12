use web_sys::{Event, HtmlElement};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub struct FormulaInput(NodeRef);

#[derive(Debug, PartialEq, Properties)]
pub struct FormulaInputProps {
    pub onchange: Callback<Event>,
}

const ID: &str = "formula-input";

impl Component for FormulaInput {
    type Message = ();

    type Properties = FormulaInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self(NodeRef::default())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <label for={ID}>
                {"Input your formula:"}
                <input
                    ref = {self.0.clone()}
                    onchange={ctx.props().onchange.clone()}
                    id={ID}
                    type="text"
                />
            </label>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.0.cast::<HtmlElement>()
                .map(|input| input.focus());
        }
    }
}
