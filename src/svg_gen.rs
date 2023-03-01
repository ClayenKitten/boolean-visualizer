use std::{collections::HashSet, iter::repeat};

use svg::{Document, node::element::{Pattern, Rectangle, Circle, Group, Text, Path, path::Data}, Node};

pub fn generate<F>(vars: &HashSet<char>, func: F) -> svg::Document
where
    F: Fn(Vec<(char, bool)>) -> bool,
{
    let mut document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(
            Pattern::new()
                .set("id", "hatch")
                .set("patternUnits", "userSpaceOnUse")
                .set("width", 4)
                .set("height", 4)
                .add(
                    Rectangle::new()
                        .set("width", 4)
                        .set("height", 4)
                        .set("fill", "white")
                )
                .add(
                    Path::new()
                        .set("d", "M-1,1 l2,-2\nM0,4 l4,-4\nM3,5 l2,-2")
                        .set("style", "stroke:gray; stroke-width:1")
                )
        );
    let all_false = vars.iter()
        .copied()
        .zip(repeat(false))
        .collect::<Vec<(char, bool)>>();
    document = document.add(background(func(all_false)));
    match vars.len() {
        0 => document,
        1 => {
            let char = *vars.iter().next().unwrap();
            document.add(
                single(
                    char,
                    func(vec![(char, true)]),
                )
            )
        },
        2 => {
            let mut vars: [char; 2] = vars.iter().copied().collect::<Vec<_>>().try_into().unwrap();
            vars.sort_unstable();
            document.add(
                double(
                    vars,
                    [
                        func(vec![(vars[0], true), (vars[1], false)]),
                        func(vec![(vars[0], true), (vars[1], true)]),
                        func(vec![(vars[0], false), (vars[1], true)]),
                    ],
                )
            )
        },
        _ => document,
    }
}

pub fn background(filled: bool) -> impl Node {
    Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("fill", if filled { "url(#hatch)" } else { "white" })
}

pub fn single(var: char, filled: bool) -> impl Node {
    Group::new()
        .add(circle("50", "50", "25", filled))
        .add(text(var, "50", "50"))
}

pub fn double(vars: [char; 2], fill: [bool; 3]) -> impl Node {
    Group::new()
        .add(circle("33.33", "50", "25", fill[0]))
        .add(circle("66.66", "50", "25", fill[2]))
        .add(intersection(fill[1]))
        .add(text(vars[0], "33.33", "50"))
        .add(text(vars[1], "66.66", "50"))
}

fn circle(cx: &str, cy: &str, r: &str, fill: bool) -> Circle {
    Circle::new()
        .set("cx", cx)
        .set("cy", cy)
        .set("r",  r)
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("fill", if fill { "url(#hatch)" } else { "white" })
        .add(
            Text::new()
                .set("x", cx)
                .set("y", cy)
        )
}

fn intersection(fill: bool) -> Path {
    Path::new()
        .set(
            "d",
            Data::new()
                .move_to((50., 31.366))
                .elliptical_arc_by((25, 25, 0, 0, 0, 0, 37.268))
                .elliptical_arc_by((25, 25, 0, 0, 0, 0, -37.268))
        )
        .set("fill", if fill { "url(#hatch)" } else { "white" })
        .set("stroke", "black")
        .set("stroke-width", 1)
}

fn text(s: impl Into<String>, x: &str, y: &str) -> impl Node {
    Text::new()
        .set("x", x)
        .set("y", y)
        .set("text-anchor", "middle")
        .set("dominant-baseline", "middle")
        .set("fill", "black")
        .set("font-family", "monospace")
        .add(svg::node::Text::new(s))
}
