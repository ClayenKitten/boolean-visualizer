mod svg_gen;
mod table_gen;
mod function;
mod bool_iterator;

use std::collections::HashMap;

use crate::function::Function;
use crate::table_gen::TruthTable;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let input = use_state(|| Function::parse("0"));
    let setter = input.setter();

    let onchange = Callback::from(move |e: Event| {
        let element: HtmlInputElement = e
            .target()
            .expect("Event should have a target when dispatched")
            .unchecked_into();
        let string = element.value();
        setter.set(Function::parse(&string));
    });

    let table = input
        .as_ref()
        .map(|func| TruthTable(
            func.vars(),
            |vals| func.eval(HashMap::from_iter(vals.into_iter())).unwrap()
        ));
    let chart = input
        .as_ref()
        .map(|func| svg_gen::generate(
            func.vars(),
            |vals| func.eval(HashMap::from_iter(vals.into_iter())).unwrap()
        ))
        .map(|chart| Html::from_html_unchecked(chart.to_string().into()));
    html! {
        <div>
            <div id="result">
                <div style="flex: 1">
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
                    {table}
                </div>
                <div>{chart}</div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
