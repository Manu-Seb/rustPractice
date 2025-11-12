use std::io;
fn main() {
    println!("Enter your string");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not input the string");

    let arr: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut answer = String::new();
    for word in arr {
        let letter = word.chars().next().unwrap();
        if is_vowel(letter) {
            answer.push_str(&format!("{}-hay ", word));
        } else {
            let mut chars = word.chars();
            let first = chars.next().unwrap();
            answer.push_str(&format!("{}-{}ay ", chars.collect::<String>(), first));
        }
    }

    println!("{}", answer);
}

fn is_vowel(c: char) -> bool {
    "aeiouAEIOU".contains(c)
}
