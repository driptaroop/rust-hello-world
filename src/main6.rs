// structure

use std::fmt::{Display, Formatter};

struct Rectangle {
    height: f32,
    width: f32
}

impl Rectangle {
    fn size(&self) -> f32 { self.width * self.height }
}

impl Rectangle {
    fn perimeter(&self) -> f32 { 2 as f32 * (self.width + self.height) }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "height: {}, width:{}", self.height, self.width) }
}

fn main() {
    let rec = Rectangle {
        height: 1.2,
        width: 2.3
    };

    println!("{}", rec);
    println!("size: {}, perimeter: {}", rec.size(), rec.perimeter());
}