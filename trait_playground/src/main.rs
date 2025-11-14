// use std::io;
mod trait_example;

mod news_and_summary;
fn main() {
    // use trait_example::functions::Largest;
    // let val = trait_example::functions::largest(1, 2);
    // println!("{}", val);
    news_and_summary::functions::test_news();

    // let mut n = String::new();

    // io::stdin().read_line(&mut n).expect("Could not read line");
    // let n = n.trim().parse().expect("Could not parse");
    //
    // let mut arr: Vec<i32> = Vec::new();
    // for _ in 0..n {
    //     let mut temp = String::new();
    //     io::stdin()
    //         .read_line(&mut temp)
    //         .expect("Could not read line");
    //     let temp = temp.trim().parse().expect("could not parse");
    //     arr.push(temp);
    // }
    //
    // let res = arr.largest();    let tup = (100, 2000);
    // let biggest = Largest::largest(&tup);
    // println!("The largest of the array is {}", res);
    // println!("The largest of the tup is {}", biggest);
}
