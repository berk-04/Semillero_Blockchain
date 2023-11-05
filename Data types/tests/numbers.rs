
#[cfg(test)]
#[allow(unused_variables)]


mod tests{

    use std::mem::size_of_val;

/*

    // Remove something to make it work
    fn test() {
        let x: i32 = 5;
        let mut y: u32 = 5;

        y = x;
        
        let z = 10; // Type of z ? 

        println!("Success!");
    }

*/
    #[test]
    fn test_1() {
        let x: i32 = 5;
        let mut _y: i32 = 5;

        _y = x;
        
        let _z = 10;

        println!("Success!");
    }

/*

    // Fill the blank
    fn main() {
        let v: u16 = 38_u8 as __;

        println!("Success!");
    }

*/

    #[test]
    fn test_2() {
        let v: u16 = 38_u8 as u16;

        println!("Success!");
    }

/*

    // Modify `assert_eq!` to make it work
    fn main() {
        let x = 5;
        assert_eq!("u32".to_string(), type_of(&x));

        println!("Success!");
    }

    // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

*/

    #[test]
    fn test_3() {
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));


        println!("Success!");
    }

    // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

/*

    // Fill the blanks to make it work
    fn main() {
        assert_eq!(i8::MAX, __); 
        assert_eq!(u8::MAX, __); 

        println!("Success!");
    }

*/

    #[test]
    // Fill the blanks to make it work
    fn test_4() {
        assert_eq!(i8::MAX, 127); 
        assert_eq!(u8::MAX, 255); 

        println!("Success!");
    }

/*

    // Fix errors and panics to make it work
    fn main() {
    let v1 = 251_u8 + 8;
    let v2 = i8::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
    }

*/

    #[test]
    // Fix errors and panics to make it work
    fn test_5() {
        let v1 = 247_u8 + 8;
        let v2 = i8::checked_add(119, 8).unwrap();   //checked_add devuelve un Option, por lo que hay que desempaquetar el valor
                                                    //unwrap() devuelve el valor si es Some, si es None, hace panic
        println!("{},{}",v1,v2);
    }
 
/*
// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
} 
*/

    #[test]
    fn test_6() {
        let c1 = 'a';
        assert_eq!(size_of_val(&c1), 4); 

        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4); 

        println!("Success!");
    }

/*

// Make it work
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

*/

    #[test]
    fn test_7() {
        let c1 = '中';
        print_char(c1);
    }

    fn print_char(c : char) {
        println!("{}", c);
    }

/*

// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    }
} 

*/

    #[test]
    fn test_8(){
        let t = false;

        if !t {
            println!("Success!");
        }    
    }

/*

// Make it work
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

*/

    #[test]
    fn test_9(){
        let f = true;
        let t = true && false;
        assert_eq!(!t,f);

        println!("Success!");
    }

/*

// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

*/

    #[test]
    fn test_10(){
        let _v: () = ();

        assert_eq!(_v, implicitly_ret_unit());
    
        println!("Success!");
    }
    
    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }
 

/*

// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("Success!");
}

*/

    #[test]
    fn test_11(){
        let unit: () = ();

        assert_eq!(size_of_val(&unit), 0);

        println!("Success!");

    }

}