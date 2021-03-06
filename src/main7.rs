use std::fmt::{Display, Formatter, Result};

fn main() {
    enum Size {
        Small, Medium, Large
    }

    let burger_size = Size::Small;

    let value = match burger_size {
        Size::Small => { "regular" }
        Size::Medium => { "classic" }
        Size::Large => { "family" }
    };

    println!("{}", burger_size as u32);

    println!("{}", value);

    // match case
    let num = 3;
    println!("{}", match num {
        1 => "hello",
        2 => "world",
        _ => "error"
    });
}