enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn _value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
} 

fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
        None => None,
        Some(value) => Some (value + 1),
    }
}