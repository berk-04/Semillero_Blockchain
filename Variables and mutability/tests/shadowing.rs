#[cfg(test)]

mod tests{
/*

    // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
    fn main() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 5);
        }

        assert_eq!(x, 12);

        let x = 42;
        println!("{}", x); // Prints "42".
    }

*/

    #[test]
    fn test(){
        let x:i32 = 5;

        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x); // Prints "42".
    }

/*

    // Remove a line in the code to make it compile
    fn main() {
        let mut x: i32 = 1;
        x = 7;
        // Shadowing and re-binding
        let x = x; 
        x += 3;


        let y = 4;
        // Shadowing
        let y = "I can also be bound to text!"; 

        println!("Success!");
    }

*/

    #[test]
    fn test_1() {
        let mut _x: i32 = 1;
        _x = 7;
        
        let _x = _x;

        let _y = 4;

        let _y = "I can also be bound to text!"; 

        println!("Success!");
    }


}