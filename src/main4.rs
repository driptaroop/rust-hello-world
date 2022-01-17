fn main(){
    let num_list1 = [1, 3, 5];
    let num_list2: [u32; 5] = [1, 3, 5, 7, 9];
    let hi_list: [&str; 10] = ["hi";10];
    println!("{}", num_list1[2]);
    println!("{}", num_list2[3]);
    println!("{}", hi_list[3]);

    let list = [1, 2, 3, 4, 5];
    for i in list {
        println!("{}", i);
    }

    for i in list.iter().rev() {
        println!("{}", i);
    }
}