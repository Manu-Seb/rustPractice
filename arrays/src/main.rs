use std::io;

fn main() {
    println!("Array Practice!!");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Enter the index you want to access");
    let mut idx = String::new();

    io::stdin()
        .read_line(&mut idx)
        .expect("Could not read line");

    let idx: usize = idx.trim().parse().expect("Could not parse string");

    let elem = arr[idx];
    println!("The value you require from the array is {elem}");
}
