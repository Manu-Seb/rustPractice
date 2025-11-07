use std::io;
fn main() {
    let mut s = String::new();
    println!("Please type your string");

    io::stdin().read_line(&mut s).expect("Could not read line");

    let word = find_first_word(&s);

    println!("{word} is the first word");
}

fn find_first_word(s: &str) -> &str {
    let items = s.as_bytes();

    for (i, &item) in items.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..s.len()];
}
