use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Please enter a number between 1 and 10!");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("an error occurred");

    let guess : u32 = guess.trim().parse().expect("an error occurred");

    let secret = rand::thread_rng().gen_range(1..=10);
    match guess.cmp(&secret){
        Ordering::Less => println!("You have guessed too little"),
        Ordering::Greater=> println!("You have guessed too large"),
        Ordering::Equal=> println!("You win"),
    }
}
