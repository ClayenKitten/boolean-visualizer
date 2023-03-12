use yew::{html, Html};

use crate::bool_iterator::BoolIterator;

#[allow(non_snake_case)]
pub fn TruthTable<F>(vars: &[char], func: F) -> Html
where
    F: Fn(&[bool]) -> bool,
{
    let rows = BoolIterator::new(vars.len() as u8)
        .map(|mut values| {
            values.reverse();
            html! {
                <tr>
                    {for values.iter().map(|val| html!(
                        <td>{if *val {"1"} else {"0"}}</td>)
                    )}
                    <td>{if func(&values) {"1"} else {"0"}}</td>
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
