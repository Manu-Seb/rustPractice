use std::collections::HashMap;

pub enum Queries {
    Insert(String),
    Read(String),
    ReadAll,
}

impl Queries {
    pub fn execute(&self, hash: &mut HashMap<String, Vec<String>>) {
        match self {
            Queries::Insert(query) => insert_db(&query, hash),
            Queries::Read(query) => read_db(&query, hash),
            Queries::ReadAll => read_all(hash),
        }
    }
}

fn insert_db(query: &String, hash: &mut HashMap<String, Vec<String>>) {
    let arr: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
    if arr.len() < 4 {
        return;
    }

    if arr[0] == "Add" && arr[2] == "to" {
        let name = &arr[1];
        let dept = &arr[3];

        hash.entry(dept.clone())
            .or_insert(Vec::new())
            .push(name.clone());
    } else {
        return;
    }
}

fn read_db(query: &String, hash: &mut HashMap<String, Vec<String>>) {
    let arr: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
    if arr.len() < 2 {
        return;
    }
    if arr[0] == "Read" {
        let dept = &arr[1];
        println!("The employees in the department {} are ", dept);
        if let Some(employees) = hash.get(dept) {
            for employee in employees {
                print!("{} ", employee);
            }
        }
        println!();
    }
}
fn read_all(hash: &HashMap<String, Vec<String>>) {
    let mut departments = hash.keys().collect::<Vec<_>>();
    departments.sort();
    println!("The employees are");

    for dept in departments {
        println!("{}", dept);
        if let Some(employees) = hash.get(dept) {
            for employee in employees {
                print!(" {} ,", employee);
            }
        }
        println!("");
    }
}
