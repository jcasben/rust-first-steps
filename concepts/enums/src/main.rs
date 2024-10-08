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
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _better_home = BetterIpAddrKind::V4(String::from("127.0.0.1"));
    let _better_loopback = BetterIpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();

    //OPTION ENUM
    let _some_number = Some(6);
    let _some_char = Some('c');
    let _absent_number: Option<i32> = None;
}
