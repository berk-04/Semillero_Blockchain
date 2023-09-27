fn main() {
    
        // initialization of tuple with data type
        let tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
        
        println!("Tuple contents = {:?}", tuple);

       
        // initialization of tuple without data type
        let tuple = ("Rust", "fun", 100);
    
        println!("Tuple contents = {:?}", tuple);
        
        //index sobre una tupla
        let random_tuple = ("saludos_utp", 200, 3.14);
    
        // accessing tuple element at index 0
        println!("Value at Index 0 = {}", random_tuple.0);
        
        // accessing tuple element at index 1
        println!("Value at Index 1 = {}", random_tuple.1);
        
        // accessing tuple element at index 2
        println!("Value at Index 2 = {}", random_tuple.2);

        //modificando valores de la tupla
    
        // Se inicializa la tupla con valores
        let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);
        
        println!("Original tuple = {:?}", mountain_heights);
        
        // Se cambian valores en la posición 3 y 4
        mountain_heights.2 = "semillero";
        mountain_heights.3 = 2023;
        
        println!("Changed tuple = {:?}", mountain_heights);

        //destructurando una tupla
        let mixture = ("Hello, World!", 16, 2.71828);
    
        // destructuring a tuple
        let (message, number, float) = mixture;          
        println!("message = {}", message);
        println!("number = {}", number);
        println!("float = {}", float);
        
        
        
}
