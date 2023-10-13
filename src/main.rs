use shapes::{circle::Circle, collisions::Collidable};

use crate::shapes::rect::Rect;
mod shapes; // make sure you use shapes.rs. First checks for a file named shapes.rs, then checks for a folder named shapes with a file called mod.rs (module)

fn main() {
    let rect = Rect::default(); // :: accesses the namespace
    let rect2 = Rect {
        x: 10.0,
        y: 10.0,
        width: 5.0,
        height: 5.0,
    };

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    let circ2 = Circle {
        x: 20.0,
        y: 20.0,
        radius: 50.0,
    };

    rect.collide(&rect2);
    circ.collide(&circ2);
    rect.collide(&circ);
}
