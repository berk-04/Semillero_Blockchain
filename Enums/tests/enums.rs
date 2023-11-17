#[cfg(test)]

//test 

enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}

//test_2 test_3 test_4

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


mod tests{

#[test]

fn test(){
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One, Number1::One);
    assert_eq!(Number1::One, Number2::One);

    println!("Success!");

}

#[test]

fn test_1(){
    let msg1 = Message::Move{__}; // Instantiating with x = 1, y = 2 
    let msg2 = Message::Write(__); // Instantiating with "hello, world!"

    println!("Success!");
}

#[test]

fn test_2(){
    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{__} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

#[test]

fn test_3(){
    let msgs: __ = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{}", msg);
}

#[test]


// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn test_4() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let __ = six {
        println!("{}", n);

        println!("Success!");
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        __ => None,
        __ => Some(i + 1),
    }
}

}

