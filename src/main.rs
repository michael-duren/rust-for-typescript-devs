use std::{fmt::Display, str::FromStr};

use shapes::{
    circle::Circle,
    collisions::{Collidable, Contains, Points},
};

use crate::shapes::rect::Rect;
use anyhow::Result;
mod shapes; // make sure you use shapes.rs. First checks for a file named shapes.rs, then checks for a folder named shapes with a file called mod.rs (module)

enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape")),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => return write!(f, "{}", c),
            Shape::Rect(r) => return write!(f, "{}", r),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rect(r) => return r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => c.contains_point(point),
            Shape::Rect(r) => r.contains_point(point),
        }
    }
}

fn main() -> Result<()> {
    let shapes = std::fs::read_to_string("shapes")? // get the file if it is okay ?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    let collisions = shapes
        .iter() // doesn't consume just refers to
        .skip(1) // don't do first one
        .zip(
            shapes
                .iter() // iterate over shapes
                .take(shapes.len() - 1), // iterator is offset
        )
        .filter(|(a, b)| a.collide(b))
        .collect::<Vec<_>>(); // collect into a vector of some sort

    for (a, b) in collisions {
        println!("Collisions: {} {}", a, b);
    }

    return Ok(());
}
