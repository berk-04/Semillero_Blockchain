use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;


enum Result<T, E> {
    Ok(T),
    Err(E),
}


// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

//The ? Operator Can Only Be Used in Functions That Return Result

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//shorter way
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

//other way
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


// fn open_file() -> Result<(), Error> {
//     let file = r#try!(File::open("hello.txt"));
//     Ok(())
// }

fn open_file() -> Result<(), Error> {
    let file = File::open("hello.txt")?;
    Ok(())
}

fn main() {
    
    //Rust group errors into two major categories: recoverable and unrecoverable errors.

    //Rust has the type Result<T, E> for recoverable errors and the panic! macro that
    //stops execution when the program encounters an unrecoverable error.





    //Unrecoverable errors with panic!

    //panic!("crash and burn"); //panic! macro to cause the error to occur and the program to stop



    //panic backtrace

    let v = vec![1, 2, 3];

    v[99]; //index out of bounds





    //Recoverable Errors with Result

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    

    //Result<T,E> with unwrap and expect

    //let greeting_file = File::open("hello.txt").unwrap(); //unwrap() method to get the value out
                                                          // of a successful result or call the panic! macro on an error,
    
    //let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt"); //expect method is similar to unwrap, but it lets us also choose the panic! error message.


    //Propagating Errors

    //when you’re writing a function whose implementation calls something that might fail,
    //instead of handling the error within this function, you can return the error to the
    //calling code so that it can decide what to do.

    //let username = read_username_from_file().expect("Failed to read username from file");

    //The ? operator

    //The ? placed after a Result value is defined to work in almost the same way as the
    //match expressions we defined to handle the Result values above.



}
