#[cfg(debug)]

//test
enum Direction {
    East,
    West,
    North,
    South,
}


//test_2

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

mod tests{

#[test]

fn test(){
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        __  => { // Matching South or North here
            println!("South or North");
        },
        _ => println!(__),
    };
}

#[test]

fn test_1(){
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = __;

    assert_eq!(binary, 1);

    println!("Success!");
}

#[test]

fn test_2(){
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

fn show_message(msg: Message) {
    match msg {
        __ => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, __);
            assert_eq!(b, __);
        }
        __ => println!("no data in these variants")
    }
}


#[test]

fn test_3(){
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(__)
    }

    println!("Success!");
}

#[test]

fn test_4(){
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

#[test]

fn test_5(){
    fn main() {
        let o = Some(7);
    
        // Remove the whole `match` block, using `if let` instead 
        match o {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
    
                println!("Success!");
            }
            _ => {}
        };
    }
    
}

#[test]

}