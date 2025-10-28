// use std::io;
fn main() {
    println!("Hello, world!");
    another_func();
    test();

    let x = mul_by_5(10);
    println!("The value that got multipled is {x}");

    // println!("Check if the value is 0");
    //
    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Could not read the value");
    //
    // let guess = guess.trim().parse().expect("could not parse");
    // if check_non_zero(guess) {
    //     println!("It is zero!!");
    // } else {
    //     println!("It is non zero");
    // }
    // looptest();
    // for_loop();
    countdown(5);
}

fn another_func() {
    println!("This is the execution of another function ");
}
fn test() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn looptest() {
    let mut init = 0;
    println!("not executing");
    let res = loop {
        init = init + 1;
        if init == 10 {
            println!("not executing naur");
            break init * 100;
        }
    };
    println!("The result is {res}");
}

fn mul_by_5(val: i32) -> i32 {
    println!("hello this is inside the function ");
    val * 10
}
// fn check_non_zero(val: i32) -> bool {
//     if val != 0 {
//         return false;
//     } else {
//         return true;
//     }
// }

fn for_loop() {
    let arr: [i32; 5] = [1; 5];
    for elem in arr {
        println!("{elem}");
    }
}

fn countdown(val: i32) {
    for i in (1..=val).rev() {
        println!("{i}");
    }
    println!("LIFTOFF");
}
