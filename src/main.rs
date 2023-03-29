mod bool_iterator;
mod formula_input;
mod function;
mod svg_gen;
mod table_gen;

use crate::formula_input::FormulaInput;
use crate::function::Function;
use crate::table_gen::TruthTable;
use function::ParseError;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let input = use_state(|| None);
    let setter = input.setter();

    let onchange = Callback::from(move |e: Event| {
        let element: HtmlInputElement = e
            .target()
            .expect("Event should have a target when dispatched")
            .unchecked_into();
        let string = element.value();

        if string.is_empty() || string.chars().all(|ch| ch.is_whitespace()) {
            setter.set(None);
        } else {
            setter.set(Some(Function::parse(&string)));
        }
    });

    html! {
        <>
            <main>
                <article>
                    <FormulaInput {onchange} />
                </article>
                {result_display(input)}
            </main>
            <footer>{"Made by\u{00A0}"}<a href="https://github.com/ClayenKitten">{"ClayenKitten"}</a></footer>
        </>
    }
}

fn result_display(formula: UseStateHandle<Option<Result<Function, ParseError>>>) -> Html {
    let func = match formula.as_ref() {
        Some(Ok(func)) => func,
        Some(Err(err)) => return error(err),
        None => return html!(),
    };
    
    let table = TruthTable(
        func.vars(),
        |vals| func.eval(vals).unwrap()
    );

    let chart = if func.vars().len() <= 3 {
        let svg = svg_gen::generate(
            func.vars(),
            |vals| func.eval(vals).unwrap()
        );
        Some(Html::from_html_unchecked(svg.to_string().into()))
    } else {
        None
    };

    html! {
        <article id="result">
            {table}
            {chart}
        </article>
    }
}

fn error(msg: impl ToString) -> Html {
    html! {
        <article class="danger">
            {format!("Error: {}.", msg.to_string())}
        </article>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
