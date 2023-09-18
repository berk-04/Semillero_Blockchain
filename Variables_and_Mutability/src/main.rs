fn main() {

    // Variables are immutable by default

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Constants and Variables
    //Constants are always immutable and Variables can change their value with the keyword mut

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Shadowing

    /*
    let y = 5;  
    y = 5; // This will throw an error
     */

    let y = 5;
    let y = y + 1; // This is allowed because we are shadowing the variable

    //inner scope

    {
        let y = y*2;
        println!("The value of y is: {y}");
    }

    println!("THe value of y is: {y}");

    
}
