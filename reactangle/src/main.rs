use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn findarea(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut width = String::new();
    let mut height = String::new();
    println!("Please enter the width and the height of the rectangle");

    io::stdin()
        .read_line(&mut width)
        .expect("could not read line");
    io::stdin()
        .read_line(&mut height)
        .expect("could not read line");
    let width: u32 = width.trim().parse().expect("Could not parse the number");
    let height: u32 = height.trim().parse().expect("Could not parse the number");

    let rec1 = Rectangle { width, height };
    let area = rec1.findarea();
    println!("The Rectangle is {rec1:?}");

    println!("The area of your rectangle is {area}");
}
