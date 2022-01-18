use std::fs::File;
use std::io::{Read, Write};

fn main() {
    //let f = File::open("hello.txt").expect("error::::");
    let f = File::open("hello.txt").unwrap();

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => panic!("there is an error opening the file, details: {}", e)
    };

    println!("{:?}", f);
}