#[cfg(test)]

fn define_x() -> String {
    "hello".to_string()
}

mod tests{
/*

    // Fix the error below with least amount of modification
    fn main() {
        let x: i32 = 10;
        {
            let y: i32 = 5;
            println!("The value of x is {} and value of y is {}", x, y);
        }
        println!("The value of x is {} and value of y is {}", x, y); 
    }

*/

    #[test]
    fn scope(){

    let x: i32 = 10;
    let y: i32 = 5;
    {
        let y = y+5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 

    }

/*

    // Fix the error with the use of define_x
    fn main() {
        println!("{}, world", x); 
    }

    fn define_x() {
        let x = "hello";
    }

*/

    #[test]
    fn main(){
        let x = define_x();
        println!("{}, world", x);
    }
    

}