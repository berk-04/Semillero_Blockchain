#[cfg(test)]

//test
struct Person {
    name: String,
    age: u8,
    hobby: String
}


//test_1
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

impl SomeTrait for Unit {  }

//test_2
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


//test_3

struct Person {
    name: String,
    age: u8,
}

//test_4 test_5

struct Person {
    name: String,
    age: u8,
}

//test_6

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//test_7

#[__]
struct Rectangle {
    width: u32,
    height: u32,
}



mod tests{

#[test]
fn test(){
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
    };

    println!("Success!");

}

#[test]
// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
fn test_1(){
    fn main() {
        let u = Unit;
        do_something_with_unit(u);
    
        println!("Success!");
    } 
    
    // Fill the blank to make the code work
    fn do_something_with_unit(u: __) {   }
    
}

#[test]

fn test_2(){
    let v = Point(__, __, __);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
 }

#[test]

fn test_3(){
    let age = 18;
    let p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18? 
    p.age = 30;

    // Fill the blank
    __ = String::from("sunfei");

    println!("Success!");
}

#[test]

fn test_4(){
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        __
    }
}

#[test]

fn test_5(){
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        __
    }
}

#[test]

fn test_6(){
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!(__, rect1); // Print debug info to stdout
}

}

