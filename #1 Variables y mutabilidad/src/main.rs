fn main() {

    // Variables son inmutables por defecto

    /*
    let y = 5;  
    y = 5; // Esto arroja un error porque y es inmutable
    */

    //Variables pueden cambiar su valor con la palabra reservada mut
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Constantes son siempre inmutables

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Shadowing

    let y = 5;
    let y = y + 1; // Esto es permitido porque estamos haciendo sombra (Shadowing) a la variable

    //inner scope - Ambito interno

    {

        let y = y*2;
        println!("The value of y is: {y}");

    }

    println!("THe value of y is: {y}");

    
}
