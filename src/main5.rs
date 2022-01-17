fn main() {
    let t1 = (1, 2, 3);
    let t2: (i32, &str, f64) = (1, "sda", 3.3321);
    println!("{}, {}", t1.0, t2.1);

    let mut vector = vec![1,2,3,4];
    vector.push(43);
    println!("{:#?}, {}", vector, vector[3]);


}