use std::collections::HashSet;

use yew::{html, Html};

use crate::bool_iterator::BoolIterator;

#[allow(non_snake_case)]
pub fn TruthTable<F>(vars: &HashSet<char>, func: F) -> Html
where
    F: Fn(Vec<(char, bool)>) -> bool,
{
    let mut vars = Vec::from_iter(vars.iter());
    vars.sort();
    let rows = BoolIterator::new(vars.len() as u8)
        .map(|values| {
            vars.iter()
                .map(|x| **x)
                .zip(values.into_iter().rev())
                .collect()
        })
        .map(|values: Vec<(char, bool)>| {
            html! {
                <tr>
                    {for values.iter().map(|(_, val)| html!(
                        <td>{if *val {"1"} else {"0"}}</td>)
                    )}
                    <td>{if func(values) {"1"} else {"0"}}</td>
                </tr>
            }
        })
        .collect::<Html>();
    html! {
        <table id="truth-table">
            <tr>
                {
                    for vars.iter()
                        .map(|h| html! {
                            <th>{h}</th>
                        })
                }
                <th>{"F"}</th>
            </tr>
            {rows}
        </table>
    }
}
