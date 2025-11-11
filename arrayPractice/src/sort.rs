use std::io;

pub fn impl_with_insertion_sort() {
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
    insertion_sort(&mut arr, n);
    println!("The array is:");
    for i in 0..n as usize {
        let number = arr.get(i);
        if let Some(num) = number {
            print!("{} ", num);
        }
    }
    println!();

    let half = &n / 2;

    print!("The median is ");
    let num = arr.get(half as usize);
    if let Some(num) = num {
        println!("{}", num);
    }
}

fn insertion_sort(arr: &mut Vec<i32>, n: u32) {
    for i in 0..n as usize {
        for j in 0..n as usize {
            let first = arr.get(i);
            let second = arr.get(j);

            if let Some(f) = first
                && let Some(s) = second
            {
                if s > f {
                    arr.swap(i, j);
                } else {
                    continue;
                }
            }
        }
    }
}
pub fn impl_with_quick_sort() {
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
    quick_sort(&mut arr, left, right);

    println!("The array is:");
    for i in 0..n as usize {
        let number = arr.get(i);
        if let Some(num) = number {
            print!("{} ", num);
        }
    }
    println!();

    let half = &n / 2;

    print!("The median is ");
    let num = arr.get(half as usize);
    if let Some(num) = num {
        println!("{}", num);
    }
}

fn quick_sort(arr: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let pivot = partition(arr, left, right);

        if pivot > 0 {
            quick_sort(arr, left, pivot -1);
        }
        quick_sort(arr, pivot + 1, right);
    }
}

fn partition(arr: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut smallest_val = left;
    for i in left..right {
        if arr[i] < pivot {
            arr.swap(i, smallest_val);
            smallest_val += 1;
        }
    }
    arr.swap(smallest_val, right);
    smallest_val
}
