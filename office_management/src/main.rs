mod types;
use std::{collections::HashMap, io};
fn main() {
    println!("Welcome to office management system");
    println!("Please enter your query");

    let mut database: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut query = String::new();
        io::stdin()
            .read_line(&mut query)
            .expect("Could not read the query");
        let query = query.trim();

        let query_type = assign_query(query);

        match query_type {
            Some(query) => query.execute(&mut database),
            _ => return,
        }
    }
}

fn assign_query(query: &str) -> Option<types::Queries> {
    let arr: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
    if arr.len() == 4 && arr[0] == "Add" && arr[2] == "to" {
        return Some(types::Queries::Insert(query.to_string()));
    } else if arr.len() == 2 && arr[0] == "Read" {
        return Some(types::Queries::Read(query.to_string()));
    } else if arr.len() == 1 && arr[0] == "Read" {
        return Some(types::Queries::ReadAll);
    } else {
        return None;
    }
}
