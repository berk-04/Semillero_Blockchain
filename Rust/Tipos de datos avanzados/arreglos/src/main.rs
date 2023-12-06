fn main() {
    // Un arreglo sin tipo de dato
    let a = [5, 4, 3, 2, 1];
    
    // Un arreglo con tipo de dato y tamaño
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Un arreglo con valores por defecto
    let c = [3; 5];
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    //Arreglo de string
    let colors = ["rojo", "verde", "azul"];
    
    // Acceso al elemento 0
    println!("1st Color: {}", colors[0]);

    // Acceso al elemento 1
    println!("2nd Color: {}", colors[1]);

    // Acceso al elemento 2
    println!("3rd Color: {}", colors[2]);


    //VECTORES
    
     // Creación de un vector
    let v = vec![1, 2, 3];
    println!("V2= {:?}", v);
    
    //Vector de string
    let colors = vec!["azul", "rojo", "verde"];
    
    // Accediendo a posiciones del vector, index directo ó con el metodo get
    println!("Primer color = {:?}", colors[0]);
    println!("Segundo color = {:?}", colors.get(1));
    println!("Tercero color = {:?}", colors.get(2));
    

}