fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let config_max = Some(3u8);
    // if let takes a pattern and an expression separated by an equal sign
    // an else can also be included
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can also define methods on enums, just like we do on structs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// null is a value that means there is no value there
// Rust replaces this concept with the generic Option<T> type
// since Option<T> and T are different types, the compiler won't let us use an Option<T> value as
// if it were a valid value
enum Option<T> {
    None,
    Some(T),
}

enum UsState {
    Alaska,
    Washington,
    Nevada,
    // and so on
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!"),
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // matches in Rust are exhaustive
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
