enum Message {
    Quit,
    Move { x: i32, y: i32, z: i32 },
    Write(String),
    ChangeColor(String),
}

struct Char {
    x: i32,
    y: i32,
    z: i32,
    color: String,
}

impl Char {
    fn action(&mut self, mes: &Message) {
        match mes {
            Message::Move { x, y, z } => {
                self.x += x;
                self.y += y;
                self.z += z;
            }
            Message::Write(s) => {
                println!(
                    "The {} man is writing {} at x: {} , y: {}, z: {}",
                    self.color, s, self.x, self.y, self.z
                )
            }
            Message::ChangeColor(s) => {
                self.color = String::from(s);
            }
            Message::Quit => println!("The {} man has quit", self.color),
        }
    }
}

fn main() {
    let mut chr = Char {
        x: 0,
        y: 0,
        z: 0,
        color: String::from("white"),
    };

    let a = Message::Move {
        x: (1),
        y: (2),
        z: (3),
    };
    let b = Message::ChangeColor(String::from("blue"));
    let c = Message::Write(String::from("LMfao you guys are cringe asf"));
    let d = Message::Quit;
    chr.action(&a);
    chr.action(&b);
    chr.action(&c);
    chr.action(&d);
}
