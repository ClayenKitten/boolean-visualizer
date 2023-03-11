mod bool_iterator;
mod function;
mod svg_gen;
mod table_gen;

use std::collections::HashMap;

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
            {formula_input(onchange)}
            {result_display(input)}
        </>
    }
}

fn formula_input(onchange: Callback<Event>) -> Html {
    html! {
        <label
            id="formula-input-label"
            for="formula-input"
        >
            {"Input your formula:"}
            <input {onchange}
                id="formula-input"
                type="text"
            />
        </label>
    }
}

fn result_display(formula: UseStateHandle<Option<Result<Function, ParseError>>>) -> Html {
    let func = match formula.as_ref() {
        Some(Ok(func)) => func,
        Some(Err(err)) => return html!(<div>{format!("Error: {err}")}</div>),
        None => return html!(),
    };
    
    let table = TruthTable(
        func.vars(),
        |vals| func.eval(HashMap::from_iter(vals.into_iter())).unwrap()
    );

    let chart = {
        let svg = svg_gen::generate(
            func.vars(),
            |vals| func.eval(HashMap::from_iter(vals.into_iter())).unwrap()
        );
        Html::from_html_unchecked(svg.to_string().into())
    };

    html! {
        <div id="result">
            <div style="flex: 1">
                {table}
            </div>
            <div>{chart}</div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
