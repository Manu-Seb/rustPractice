#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
}

fn main() {
    let add1 = IpAddr {
        kind: IpAddrKind::V4(String::from("128.1.3.4")),
    };

    let add2 = IpAddr {
        kind: IpAddrKind::V6(String::from("128.1.3.4")),
    };

    println!("The values of address 1 is {:?}", add1);
    println!("The values of address 1 is {:?}", add2);
}
