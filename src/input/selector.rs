use super::InputKind;
use yew::{function_component, html, Properties, Callback};

#[function_component]
pub fn InputKindSelector(props: &Props) -> yew::Html {
    html! {
        <menu id="input-kind-selector">
            <button class="selected" onclick = {props.onselect.reform(|_| InputKind::Text)}>
                <img class="icon" src="static/icons/text.svg"/>
            </button>
            <button onclick = {props.onselect.reform(|_| InputKind::Graph)}>
                <img class="icon" src="static/icons/graph.svg"/>
            </button>
        </menu>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub onselect: Callback<InputKind>,
}
