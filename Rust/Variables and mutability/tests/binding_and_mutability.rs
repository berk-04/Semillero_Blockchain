#[cfg(test)]

mod tests{
    
/*

    // Fix the error below with least amount of modification to the code
    fn main() {
        let x: i32; // Uninitialized but used, ERROR !
        let y: i32; // Uninitialized but also unused, only a Warning !

        assert_eq!(x, 5);
        println!("Success!");
    }

 */
    #[test]
    fn test() {
        let x: i32 = 5; // Uninitialized but used, ERROR !
        let _y: i32; // Uninitialized but also unused, only a Warning !

        assert_eq!(x, 5);
        println!("Success!");
    }

/*

    // Fill the blanks in the code to make it compile
    fn main() {
        let __ __ = 1;
        __ += 2; 
        
        assert_eq!(x, 3);
        println!("Success!");
    }

*/

    #[test]
    fn test_1(){
        let mut x = 1;
        x += 2; 
    
        assert_eq!(x, 3);
        println!("Success!");

    }

    #[test]
    fn test_variables6() {
        const NUMBER: u8 = 3;
        println!("Number {}", NUMBER);
    }
}