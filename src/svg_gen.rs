use std::{collections::HashSet, iter::repeat};

use svg::{Document, node::{element::{Pattern, Path, Rectangle, Circle, Group, ClipPath, Definitions, Use, Text}}, Node};

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
        .add(circle("50%", "50%", "25", filled))
        .add(text(var, "50", "50"))
}

pub fn double(vars: [char; 2], fill: [bool; 3]) -> impl Node {
    Group::new()
        .add(
            Definitions::new()
                .add(
                    circle("33", "50", "25", fill[0])
                        .set("id", "circle_left")
                )
                .add(
                    circle("66", "50", "25", fill[2])
                        .set("id", "circle_right")
                )
                .add(
                    ClipPath::new()
                        .set("id", "clip1")
                        .add(circle("33", "50", "25.5", false))
                )
                .add(
                    ClipPath::new()
                        .set("id", "clip2")
                        .add(circle("33", "50", "24.5", false))
                )
        )
        .add(Use::new().set("href", "#circle_left"))
        .add(Use::new().set("href", "#circle_right"))
        .add(text(vars[0], "33", "50"))
        .add(text(vars[1], "66", "50"))
        .add(
            Circle::new()
                .set("cx", 66)
                .set("cy", 50)
                .set("r",  25.5)
                .set("fill", "black")
                .set("clip-path", "url(#clip1)")
        )
        .add(
            Circle::new()
                .set("cx", 66)
                .set("cy", 50)
                .set("r",  24.5)
                .set("fill", if fill[1] { "url(#hatch)" } else { "white" })
                .set("clip-path", "url(#clip2)")
                .add(text("F", "50", "50"))
        )
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
