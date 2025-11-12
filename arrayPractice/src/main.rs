use std::{collections::HashMap, io};
mod sort;
use crate::sort::Sort;

fn main() {
    println!("enter the number of digits");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Could not read line");
    let n: u32 = n.trim().parse().expect("Could not parse");

    println!("enter the values");
    let mut arr: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut val = String::new();
        io::stdin()
            .read_line(&mut val)
            .expect("Could not read line");
        let val: i32 = val.trim().parse().expect("Could not parse");

        arr.push(val);
    }
    let left = 0;
    let right = arr.len() - 1;

    println!("Select 1 for quick sort and 2 for insertion sort");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("could not read line");
    sort::quick_sort(&mut arr, left, right);

    let choice: u32 = choice.trim().parse().expect("Could not parse message");
    let mut method = match find_method(choice, arr) {
        Some(m) => m,
        None => {
            println!("not a valid choice");
            return;
        }
    };

    method.sort();

    let arr = match method {
        Sort::Quick(ref arr) | Sort::Insertion(ref arr) => arr,
    };

    print_arr(&arr);
    let half = &n / 2;

    print!("The median is ");
    let num = arr.get(half as usize);
    if let Some(num) = num {
        println!("{}", num);
    }

    let modevalue = mode(arr);
    println!("The mode of the array is {}", modevalue);

    
}

fn print_arr(arr: &Vec<i32>) {
    println!("The array is:");
    let n = arr.len();
    for i in 0..n as usize {
        let number = arr.get(i);
        if let Some(num) = number {
            print!("{} ", num);
        }
    }
    println!();
}

fn find_method(val: u32, arr: Vec<i32>) -> Option<Sort> {
    match val {
        1 => return Some(Sort::Quick(arr)),
        2 => return Some(Sort::Insertion(arr)),
        _ => None,
    }
}

fn mode(arr: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();

    let mut curr_mode = arr[0];
    let mut curr_count = 0;
    for &i in arr {
        let count = hash.entry(i).or_insert(0);
        *count += 1;
        if *count > curr_count {
            curr_count = *count;
            curr_mode = i;
        }
    }
    curr_mode
}
