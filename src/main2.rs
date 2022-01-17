use std::io;

fn main() {
    let mut text = String::new();
    println!("type something:");
    io::stdin().read_line(&mut text).expect("failed");
    let number: u32 = text.trim().parse().expect("cannot parse");
    let number1 = text.trim().parse::<i32>().expect("error while parsing");
    println!("you typed {} which is parsed as {} and {}", text, number, number1);
}