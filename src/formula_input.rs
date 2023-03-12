use web_sys::{Event, HtmlElement, HtmlInputElement};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub struct FormulaInput {
    input: NodeRef,
    value: String,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    OnInput,
    OnChange(Event),
}

const ID: &str = "formula-input";

impl Component for FormulaInput {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: String::new(),
            input: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInput => {
                let input = self.input.cast::<HtmlInputElement>().unwrap();
                let value = input.value();
                self.value = value
            }
            Msg::OnChange(event) => {
                ctx.props().onchange.emit(event);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <label for={ID}>
                {"Input your formula:"}
                <input
                    ref = {self.input.clone()}
                    id={ID}
                    type="text"
                    oninput={ctx.link().callback(|_| Msg::OnInput)}
                    onchange={ctx.link().callback(Msg::OnChange)}
                />
                <pre aria-hidden="true">
                    {highlighting(self.value.as_str())}
                </pre>
            </label>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.input.cast::<HtmlElement>().map(|input| input.focus());
        }
    }
}

fn highlighting(input: &str) -> Html {
    let highlighted = input
        .chars()
        .map(|ch| match ch {
            '&' | '|' | '!' => html!(<span class="operator">{ch}</span>),
            '(' | ')' => html!(<span class="bracket">{ch}</span>),
            'a'..='z' | '0' | '1' => html!(<span class="variable">{ch}</span>),
            _ => html!(<span class="error">{ch}</span>),
        })
        .collect::<Html>();
    html!(<code>{highlighted}</code>)
}
