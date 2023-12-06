
#[cfg(test)]
#[allow(unused_variables)]

mod tests{

/*

// Fix error without adding new line
fn main() {
    let s: str = "hello, world";

    println!("Success!");
}

*/

#[test]
fn test() {
    let s: &str = "hello, world";

    println!("Success!");
}

/*

// Fix the error with at least two solutions
fn main() {
let s: Box<str> = "hello, world".into(); //Box is a smart pointer that points to data on the heap
greetings(s)
}

fn greetings(s: &str) {
println!("{}",s)
}

*/
#[test]
fn test_1() {
    let s: Box<str> = "hello, world".into(); //Box is a smart pointer that points to data on the heap
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

/*

// Fill the blank
fn main() {
let mut s = __;
s.push_str("hello, world");
s.push('!');

assert_eq!(s, "hello, world!");

println!("Success!");
}

*/

#[test]
fn test_2() {
    //let mut s = "".to_string();
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}


/*

// Fix all errors without adding newline
fn main() {
let s = String::from("hello");
s.push(',');
s.push(" world");
s += "!".to_string();

println!("{}", s);
}

*/

#[test]
fn test_3() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

/*

// Fill the blank
fn main() {
let s = String::from("I like dogs");
// Allocate new memory and store the modified string there
let s1 = s.__("dogs", "cats");

assert_eq!(s1, "I like cats");

println!("Success!");
}


*/

#[test]
fn test_4() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

/*


// Fix errors without removing any line
fn main() {
let s1 = String::from("hello,");
let s2 = String::from("world!");
let s3 = s1 + s2; 
assert_eq!(s3, "hello,world!");
println!("{}", s1);
}

*/

#[test]
fn test_5() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}


}