#[cfg(test)]

mod tests{

/*

    // Fix the error below with least amount of modification
    fn main() {
        let (x, y) = (1, 2);
        x += 2;

        assert_eq!(x, 3);
        assert_eq!(y, 2);

        println!("Success!");
    }

*/
    #[test]
    fn test() {
        let (mut x, y) = (1, 2);
        x += 2;

        assert_eq!(x, 3);
        assert_eq!(y, 2);

        println!("Success!");
    }
/*

    fn main() {
        let (x, y);
        (x,..) = (3, 4);
        [.., y] = [1, 2];
        // Fill the blank to make the code work
        assert_eq!([x,y], __);

        println!("Success!");
    } 

*/

    #[test]
    fn test_1() {
        let (x, y);
        (x,..) = (3, 4);
        [.., y] = [1, 2];

        assert_eq!([x,y], [3,2]);

        println!("Success!");
    } 


}