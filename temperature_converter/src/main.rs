use std::io;
fn main() {
    println!("Select 1 if you want to convert farenheit_to_celcius");
    println!("Select 2 if you want to convert celcius_to_farenheit");

    let mut opt = String::new();
    io::stdin()
        .read_line(&mut opt)
        .expect("Could not read line");

    let opt: u32 = opt.trim().parse().expect("could not convert number");

    println!("enter the value you want to convert");
    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Could not read line");

    let val: f64 = val.trim().parse().expect("could not convert number");

    let mut res: f64 = 0.0;

    if opt == 1 {
        res = farenheit_to_celcius(val);
    } else if opt == 2 {
        res = celcius_to_farenheit(val);
    } else {
        println!("coult not compute");
    }
    println!("The converted value is {}", res);
}

fn celcius_to_farenheit(val: f64) -> f64 {
    let res = (1.8 * val) + 32.0;
    return res;
}

fn farenheit_to_celcius(val: f64) -> f64 {
    let res = 0.5 * (val - 32.0);
    return res;
}
