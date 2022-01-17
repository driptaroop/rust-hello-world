fn main1() {
    let num1: i8 = 1;
    let num2: i8 = -9;
    let mut sum = num1 + num2;
    println!("sum:{}", sum);

    sum = num1 - num2;
    println!("subtract: {}", sum);

    const PI: f64 = 3.14;
    println!("PI is {}", PI);

    let flag: bool = 1 == 1;
    println!("1 == 1 : {}, 1 != 2: {}", flag, 1 != 2);

    if  2 <= 3 {
        println!("if true");
    }

    if 2>=3 {
        println!("if false");
    } else {
        println!("else true");
    }


    if 3>3 {
        println!("if false");
    } else if 3<3 {
        println!("else if false");
    } else {
        println!("else true");
    }

    // if as expression
    let char01 = 'A';
    let result01 = if char01=='A' { 2 } else { 3 };
    println!("result 01: {}", result01);

    // loop
    let mut number01: i8 = 1;
    while number01 < 5 {
        println!("in loop 01");
        number01 += 1;
    }

    loop {
        println!("in loop 02");
        number01 += 1;
        if number01 >= 10 { 
            break; 
        }
    }

    let number02 = 3;
    println!("result 03 : {}", times_two(number02));
    println!("result 04 : {}", times_three(number02));
    println!("original number: {}", number02);

    let temp = Template{
        x: 4,
        y: 5.5443,
        flag: false
    };

    println!("y in template is {}", temp.y);


    // string module
    let mut str01 = String::from("hello");
    str01.push(' ');
    println!("{}", str01 + "world");
}

fn times_two(i: i32) -> i32 {
    return i * 2;
}
fn times_three(i: i32) -> i32 {
    i * 3
}

struct Template {
    x: u32,
    y: f64,
    flag: bool
}