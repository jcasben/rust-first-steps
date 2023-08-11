enum IpAddrKind {
    V4,
    V6,
}

enum BetterIpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let better_home = BetterIpAddrKind::V4(String::from("127.0.0.1"));
    let better_loopback = BetterIpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();

    //OPTION ENUM
    let some_number = Some(6);
    let some_char = Some('c');
    let absent_number: Option<i32> = None;
}
