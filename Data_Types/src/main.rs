fn main() {
    //Scalar Types
    //integers, floating-point numbers, Booleans, and characters

    //Integer Types
    //Length	Signed	Unsigned
    //8-bit    	i8	      u8
    //16-bit	i16	      u16
    //32-bit	i32    	  u32
    //64-bit	i64	      u64
    //128-bit	i128	  u128
    //arch	    isize	  usize

    //Signed -> -(2^(n-1)) to 2^(n-1) - 1
    //Unsigned -> 0 to 2^n - 1

    //integers literals

    //Compound Types

    //tuple
    //is a general way of grouping together a number of values with
    //a variety of types into one compound type

    let tuple: (u8, i32, f64) = (6, 257, 5.78);
    let (x,y,z) = tuple;

    println!("y variable is: {y}");

    let variable1: u8 = tuple.0;
    let variable2: i32 = tuple.1;
    let variable3: f64 = tuple.2;
    
    println!("variable1 is: {variable1}");
    println!("variable2 is: {variable2}");
    println!("variable3 is: {variable3}");

    //array
    //in an array every element of an array must have the same type

    let array = [1,2,3,4,5];

    let array1: [u8; 5] = [1,2,3,4,5];

    let array2 = [3; 5];

    println!("Array: {:?}", array2)

}   
