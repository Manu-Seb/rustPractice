pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, second: &Rectangle) -> bool {
        self.length > second.length && self.width > second.width
    }
}

fn greeting(name: &str) -> String {
    format!("Hello to you, {}", name)
}

fn add_a_hundred(val: i32) -> i32 {
    if val < 1 || val > 100 {
        panic!("the value should be between 1 and 100");
    } else {
        val + 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rec1: Rectangle = Rectangle {
            length: 32,
            width: 30,
        };
        let rec2: Rectangle = Rectangle {
            length: 30,
            width: 20,
        };

        assert!(rec1.can_hold(&rec2));
    }
    #[test]
    fn smaller_cant_hold_larger() {
        let rec1: Rectangle = Rectangle {
            length: 29,
            width: 30,
        };
        let rec2: Rectangle = Rectangle {
            length: 30,
            width: 20,
        };

        assert!(!rec1.can_hold(&rec2));
    }

    #[test]
    fn doesnt_work() {
        panic!("does not work");
    }
    #[test]
    fn greetings_has_name() {
        let name = "carol";
        let res = greeting(name);
        assert!(res.contains(name));
    }

    #[test]
    #[should_panic]
    fn test_hundred() {
        add_a_hundred(110);
    }
}
