
#[derive(Debug)] //to print the struct
struct User {
    name: String,
    age: u8,
    email: String,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    //associated function
    //it doesn't take self as a parameter and they are often used for constructors that will return a new instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Color(u8, u8, u8); //tuple struct

struct UnitLikeStruct; //unit-like struct



fn main() {
    //struct is a compound data type

    //mutable struct

    let user1 = User {
        name: String::from("Juan"),
        age: 25,
        email: String::from("example@gmail.com"),
        active: true,
    };

    println!("User1 name: {}", user1.name);

   //inmutable struct

    let _user2 = build_user(String::from("example@gmail.com"), String::from("Pedro"));

    //user2.age = 30; //error: cannot assign to immutable field `user2.age`



    //struct update syntax

    let user3 = User {
        email: String::from("example1@gmail.com"),
        ..user1 //move the rest of the fields from user1 to user3, so we can't use user1 anymore

    };
    
    //println!("User1 name: {}", user1.name);
    println!("User3 name: {}", user3.name);


    //tuple structs

    let color = Color(255, 123, 255);


    //unit-like structs

    let _unit_like_struct = UnitLikeStruct;

    //print the structures for debugging porpuses

    println!("User1: {:?}", user3);


    //method syntax

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    //associated functions



}

//with this functions we can create a new struct with inmutable values (age and active)
fn build_user(email: String, name: String) -> User {
    User {
        name, //name: name,  init shorthand
        age: 25,
        email, //email: email, init shorthand
        active: true,
    }
}
