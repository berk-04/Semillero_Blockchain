#[cfg(test)]

mod tests{

/*

fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}

*/

#[test]

fn test() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = &x;
    println!("{},{}",x,y);
}

/*
// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
}

*/
#[test]

fn test_1() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

/*

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.into_bytes();
    s
}

*/

#[test]

fn test_2() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
}

/*
// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

*/

#[test]

fn test_3() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}

/*
// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

*/

#[test]
fn test_4() {
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);
}


/*

fn main() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let s1 = s;

    s1.push_str("world");

    println!("Success!");
}

*/

#[test]

fn test_5() {
    let s = String::from("hello, ");
    
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}


/*

fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}

*/

#[test]

fn test_6() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    assert_eq!("world", t.1);
 }

/*

fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (__, __) = __;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

*/

#[test]

fn test_7() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2) = t.clone();

    assert_eq!("hello", s1);
    assert_eq!("world", s2);
    assert_eq!(("hello".to_string(), "world".to_string()), t);

 }
 
}