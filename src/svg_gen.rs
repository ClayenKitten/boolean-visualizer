use std::{collections::HashSet, f64::consts::FRAC_PI_2, iter::repeat};

use svg::{
    node::{
        element::{path::Data, Circle, Group, Path, Pattern, Rectangle, Text},
        Value,
    },
    Document, Node,
};

pub fn generate<F>(vars: &HashSet<char>, func: F) -> svg::Document
where
    F: Fn(Vec<(char, bool)>) -> bool,
{
    let mut document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(fill_pattern());
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

/// A hatching pattern that is used to fill circles and their intersections.
fn fill_pattern() -> svg::node::element::Pattern {
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
        .add(text("50", "50", var))
}

pub fn double(vars: [char; 2], fill: [bool; 3]) -> impl Node {
    Group::new()
        .add(circle("33.33", "50", "25", fill[0]))
        .add(circle("66.66", "50", "25", fill[2]))
        .add(
            intersection(
                Pos { x: 100./3., y: 50. },
                Pos { x: 200./3., y: 50. },
                25.,
                fill[1],
            )
        )
        .add(text("33.33", "50", vars[0]))
        .add(text("66.66", "50", vars[1]))
}

fn circle(
    cx: impl Into<Value>,
    cy: impl Into<Value>,
    r: impl Into<Value>,
    fill: bool
) -> Circle {
    Circle::new()
        .set("cx", cx)
        .set("cy", cy)
        .set("r", r)
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("fill", if fill { "url(#hatch)" } else { "white" })
}

fn intersection(
    circle1: Pos,
    circle2: Pos,
    radius: f64,
    fill: bool,
) -> Path {
    let distance = Pos::distance(circle1, circle2);
    let center = Pos::center(circle1, circle2);
    let angle = (circle1.y - circle2.y).atan2(circle1.x - circle2.x) + FRAC_PI_2;
    // Length of rhombus's diagonal by its side and another diagonal
    let length = f64::sqrt(4. * radius.powi(2) - distance.powi(2));
    let start = {
        let x = center.x - (length/2.) * angle.cos();
        let y = center.y - (length/2.) * angle.sin();
        Pos { x, y }
    };
    let dx = length * angle.cos();
    let dy = length * angle.sin();
    Path::new()
        .set(
            "d",
            Data::new()
                .move_to((start.x, start.y))
                .elliptical_arc_by((radius, radius, 0, 0, 0, dx, dy))
                .elliptical_arc_to((radius, radius, 0, 0, 0, start.x, start.y)),
        )
        .set("fill", if fill { "url(#hatch)" } else { "white" })
        .set("stroke", "black")
        .set("stroke-width", 1)
}

fn text(
    x: impl Into<Value>,
    y: impl Into<Value>,
    s: impl Into<String>,
) -> impl Node {
    Text::new()
        .set("x", x)
        .set("y", y)
        .set("text-anchor", "middle")
        .set("dominant-baseline", "middle")
        .set("fill", "black")
        .set("font-family", "monospace")
        .add(svg::node::Text::new(s))
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Pos {
    x: f64,
    y: f64,
}

impl Pos {
    /// Computes the distance between points.
    pub fn distance(self, other: Self) -> f64 {
        f64::sqrt(
            (self.x - other.x).powi(2) +
            (self.y - other.y).powi(2)
        )
    }

    /// Computes a point that is in the middle between two provided.
    pub fn center(self, other: Self) -> Self {
        Pos {
            x: (self.x + other.x) / 2.,
            y: (self.y + other.y) / 2.,
        }
    }
}
