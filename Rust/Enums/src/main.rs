
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind2{
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    //function to show the message
    fn call(&self) {
        match self{
            Message::Write(text) => println!("{}", text),
            _ => println!("No message"),
        }
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    //first way

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //second way

    let _home = IpAddrKind2::V4(String::from("127.0.0.1"));

    let _loopback = IpAddrKind2::V6(String::from("::1"));

    //implementing methods
    
    let m = Message::Write(String::from("hello"));
    m.call();


    //Option enum

    //The option enum is defined by the standard library

    let some_number = Some(5);
    let some_string = Some("a string");

    //If we use None rather than Some, we need to tell Rust what type of Option<T> we have

    let absent_number: Option<i32> = None;

    //Usages of Option
    //if an argument of the function is optional,
    //If the function is non-void and if the output it returns can be empty,
    //If the value, of a property of the data type can be empty, We have to use their data type as an Option type





    //The match Control Flow Operator

    //The match operator is used to compare a value against a series of patterns
    // and then execute code based on which pattern matches

    let coin = Coin::Quarter(UsState::Alaska);

    match coin {
        Coin::Penny => println!("Lucky penny!"),
        Coin::Nickel => println!("Not so lucky"),
        Coin::Dime => println!("Not so lucky"),
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
        },
    }

    //match with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    //Catch all with _

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("Not one, three or five"),
    }

}
