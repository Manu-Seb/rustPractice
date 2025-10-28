fn main() {
    let arr = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "
        Twelve drummers drumming
        ",
        "
        Eleven pipers piping
        ",
        "
        Ten lords a-leaping
        ",
        "
        Nine ladies dancing
        ",
        "
        Eight maids a-milking
        ",
        "
        Seven swans a-swimming
        ",
        "
        Six geese a-laying
        ",
        "
        Five golden rings
        ",
        "
        Four calling birds
        ",
        "
        Three French hens
        ",
    ];

    for i in 1..=12 {
        println!("On the {i}th day of christmas");
        println!("My true love gave to me");
        for j in 0..i {
            println!("{}", arr[j]);
        }
    }
}
