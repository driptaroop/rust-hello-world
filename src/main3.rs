fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // error here
    //println!("{}", s1);
    println!("{}", s2);
    let s3 = s2.clone();
    println!("{}", s3);

    let mut s4 = String::from("hi");
    let len = calculate_length(&s4);
    println!("{} length is {}", s4, len);
    add_name(&mut s4);
    println!("{}", s4);

    let s5 = String::from("hello!!!!");
    println!("{}: & -> {}", s5, &s5);
    let s6 = &s5;
    println!("{}: & -> {}", s5, &s5);
    println!("{}: & -> {}", s6, &s6);
}

fn calculate_length(s: &String)-> usize {
    s.len()
}

fn add_name(s: &mut String){
    s.push_str(" Dripto");
}