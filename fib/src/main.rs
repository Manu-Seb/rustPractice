use std::io;
fn main() {
    println!("Let us find the nth fibonacci number");
    println!("enter the value of 'n'");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("could not read line");

    let n: u32 = n.trim().parse().expect("could not parse line");

    let mut first = 0;
    let mut second = 1;

    for _i in 2..n {
        let temp: u32 = first;
        first = second;
        second = second + temp;
    }

    println!("the value required is {}", second);
}
