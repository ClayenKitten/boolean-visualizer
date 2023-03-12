use std::f64::consts::FRAC_PI_2;

use svg::{
    node::element::{path::Data, Circle, Group, Path, Pattern, Rectangle, Text},
    Document, Node,
};

pub fn generate<F>(vars: &[char], func: F) -> svg::Document
where
    F: Fn(&[bool]) -> bool,
{
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(fill_pattern())
        .add(background(func(&vec![false; vars.len()])));
    match vars.len() {
        0 => document,
        1 => {
            let char = *vars.iter().next().unwrap();
            document.add(single(char, func(&[true])))
        }
        2 => {
            let vars: [char; 2] = vars.to_vec().try_into().unwrap();
            document.add(double(
                vars,
                [
                    func(&[true, false]),
                    func(&[true, true]),
                    func(&[false, true]),
                ],
            ))
        }
        3 => {
            let vars: [char; 3] = vars.to_vec().try_into().unwrap();
            document.add(triple(
                vars,
                [
                    func(&[true, false, false]),
                    func(&[false, true, false]),
                    func(&[false, false, true]),
                    func(&[true, true, false]),
                    func(&[true, false, true]),
                    func(&[false, true, true]),
                    func(&[true, true, true]),
                ],
            ))
        }
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
                .set("fill", "white"),
        )
        .add(
            Path::new()
                .set("d", "M-1,1 l2,-2\nM0,4 l4,-4\nM3,5 l2,-2")
                .set("style", "stroke:gray; stroke-width:1"),
        )
}

fn background(filled: bool) -> impl Node {
    Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("fill", if filled { "url(#hatch)" } else { "white" })
}

const RADIUS: f64 = 25.;

fn single(var: char, fill: bool) -> impl Node {
    Group::new()
        .set("transform", "translate(50, 50)")
        .add(circle(Pos::ZERO, fill))
        .add(text(0., 0., var))
}

fn double(vars: [char; 2], fill: [bool; 3]) -> impl Node {
    let c1 = Pos { x: -RADIUS / 2., y: 0. };
    let c2 = Pos { x: RADIUS / 2., y: 0. };

    Group::new()
        .set("transform", "translate(50, 50)")
        .add(circle(c1, fill[0]))
        .add(circle(c2, fill[2]))
        .add(intersection2(c1, c2, fill[1]))
        .add(text(-RADIUS * (15. / 16.), 0., vars[0]))
        .add(text(RADIUS * (15. / 16.), 0., vars[1]))
}

fn triple(vars: [char; 3], fill: [bool; 7]) -> impl Node {
    let dy = RADIUS * f64::sqrt(3.) / 6.;

    let c1 = Pos { x: 0., y: -2. * dy };
    let c2 = Pos { x: -RADIUS / 2., y: dy };
    let c3 = Pos { x: RADIUS / 2., y: dy };

    Group::new()
        .set("transform", "translate(50, 50)")
        .add(circle(c1, fill[0]))
        .add(circle(c2, fill[1]))
        .add(circle(c3, fill[2]))
        .add(intersection2(c1, c2, fill[3]))
        .add(intersection2(c1, c3, fill[4]))
        .add(intersection2(c2, c3, fill[5]))
        .add(intersection3(fill[6]))
        .add(text(c1.x, c1.y - 12., vars[0]))
        .add(text(c2.x - 9., c2.y + 9., vars[1]))
        .add(text(c3.x + 9., c3.y + 9., vars[2]))
}

fn circle(center: Pos, fill: bool) -> Circle {
    Circle::new()
        .set("cx", center.x)
        .set("cy", center.y)
        .set("r", RADIUS)
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("fill", if fill { "url(#hatch)" } else { "white" })
}

fn intersection2(circle1: Pos, circle2: Pos, fill: bool) -> Path {
    let distance = Pos::distance(circle1, circle2);
    let center = Pos::center(circle1, circle2);
    let angle = (circle1.y - circle2.y).atan2(circle1.x - circle2.x) + FRAC_PI_2;
    // Length of rhombus's diagonal by its side and another diagonal
    let length = f64::sqrt(4. * RADIUS.powi(2) - distance.powi(2));
    let start = {
        let x = center.x - (length / 2.) * angle.cos();
        let y = center.y - (length / 2.) * angle.sin();
        Pos { x, y }
    };
    let dx = length * angle.cos();
    let dy = length * angle.sin();
    Path::new()
        .set(
            "d",
            Data::new()
                .move_to((start.x, start.y))
                .elliptical_arc_by((RADIUS, RADIUS, 0, 0, 0, dx, dy))
                .elliptical_arc_to((RADIUS, RADIUS, 0, 0, 0, start.x, start.y)),
        )
        .set("fill", if fill { "url(#hatch)" } else { "white" })
        .set("stroke", "black")
        .set("stroke-width", 1)
}

fn intersection3(fill: bool) -> Path {
    let pos1 = Pos {
        x: 0.,
        y: -f64::sqrt((RADIUS / 2.).powi(2) + (3f64.sqrt() / 6. * RADIUS).powi(2)),
    };
    let pos2 = Pos {
        x: RADIUS / 2.,
        y: 3f64.sqrt() * RADIUS / 6.,
    };
    let pos3 = Pos {
        x: -RADIUS / 2.,
        y: 3f64.sqrt() * RADIUS / 6.,
    };
    Path::new()
        .set(
            "d",
            Data::new()
                .move_to((pos1.x, pos1.y))
                .elliptical_arc_to((RADIUS, RADIUS, 0, 0, 1, pos2.x, pos2.y))
                .elliptical_arc_to((RADIUS, RADIUS, 0, 0, 1, pos3.x, pos3.y))
                .elliptical_arc_to((RADIUS, RADIUS, 0, 0, 1, pos1.x, pos1.y)),
        )
        .set("fill", if fill { "url(#hatch)" } else { "white" })
        .set("stroke", "black")
        .set("stroke-width", 1)
}

fn text(x: f64, y: f64, s: impl Into<String>) -> impl Node {
    Group::new()
        .add(
            Rectangle::new()
                .set("x", x - 5.)
                .set("y", y - 5.)
                .set("width", 10)
                .set("height", 10)
                .set("rx", 1)
                .set("ry", 1)
                .set("fill", "white")
                .set("stroke", "black")
                .set("stroke-width", 1),
        )
        .add(
            Text::new()
                .set("x", x)
                .set("y", y)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("fill", "black")
                .set("font-size", "7")
                .set("font-family", "\"andale mono\", monospace")
                .add(svg::node::Text::new(s)),
        )
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Pos {
    x: f64,
    y: f64,
}

impl Pos {
    pub const ZERO: Pos = Pos { x: 0., y: 0. };

    /// Computes the distance between points.
    pub fn distance(self, other: Self) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2))
    }

    /// Computes a point that is in the middle between two provided.
    pub fn center(self, other: Self) -> Self {
        Pos {
            x: (self.x + other.x) / 2.,
            y: (self.y + other.y) / 2.,
        }
    }
}
