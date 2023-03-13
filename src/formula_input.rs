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
            <>
                <label for="formula-input-inner">{"Input your formula:"}</label>
                <div id="formula-input">
                    <input
                        ref = {self.input.clone()}
                        type="text"
                        id="formula-input-inner"
                        oninput={ctx.link().callback(|_| Msg::OnInput)}
                        onchange={ctx.link().callback(Msg::OnChange)}
                    />
                    <pre aria-hidden="true">
                        {highlighting(self.value.as_str())}
                    </pre>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.input.cast::<HtmlElement>().map(|input| input.focus());
        }
    }
}

fn highlighting(input: &str) -> Html {
    enum Entry {
        Variable(char),
        Operator(char),
        Bracket {
            depth: i32,
            is_left: bool,
        },
        Unknown(char),
    }

    let mut buffer = Vec::<Entry>::with_capacity(input.len());
    let mut depth = 0i32;
    for ch in input.chars() {
        buffer.push(match ch {
            'a'..='z' | '0' | '1'
                => Entry::Variable(ch),
            '&' | '|' | '!'
                => Entry::Operator(ch),
            '(' => {
                let entry = Entry::Bracket { depth, is_left: true };
                depth += 1;
                entry
            },
            ')' => {
                depth -= 1;
                Entry::Bracket { depth, is_left: false }
            },
            _ => Entry::Unknown(ch),
        })
    }
    let highlighted = buffer.into_iter()
        .map(|entry| match entry {
            Entry::Variable(var)
                => html!(<span class="variable">{var}</span>),
            Entry::Operator(op)
                => html!(<span class="operator">{op}</span>),
            Entry::Bracket { depth: mut cur_depth, is_left } => {
                let invalid = cur_depth < 0 ||
                    is_left && cur_depth < depth;
                if depth < 0 {
                    cur_depth += -depth;
                }
                html! {
                    <span
                        class={if invalid {"bracket error"} else {"bracket"}}
                        data-depth={(cur_depth % 3).abs().to_string()}
                    >
                        {if is_left {'('} else {')'}}
                    </span>
                }
            }
            Entry::Unknown(ch)
                => html!(<span class="error">{ch}</span>),
        })
        .collect::<Html>();
    html!(<code>{highlighted}</code>)
}
