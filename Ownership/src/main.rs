fn main() {
    //The stack have a fixed size (LIFO)

    //The heap is a memory that can grow in size 

    let _s = "Hola mundo"; //String literal have a fixed size (stack)

    let mut s = String::from("Hola mundo"); //String type have a variable size (heap)

    s.push_str(", soy un string");

    println!("{}", s);





    //Copy

    let s1 = String::from("Hola mundo"); //s1 is valid
    let _s2 = s1;                      //s1 is not valid, s2 is valid (move)

    //println!("{}, {}", s1, s2); //Error, s1 is not valid





    //Clone

    let s1 = String::from("Hola mundo"); //s1 is valid
    let s2 = s1.clone();                //s1 and s2 are valid (clone)

    println!("{}, {}", s1, s2); //s1 and s2 are valid

    //In rust we have 'move' and 'deep copy' (clone) unlike other languages that have 'shallow copy'

    //References

    let s1 = String::from("Hola mundo"); //s1 is valid
    let len = calculate_length(&s1);     //s1 is valid

    println!("La longitud de '{}' es {}.", s1, len); //s1 is valid




    //mut references

    let mut s = String::from("Hola mundo"); //s is valid
    change(&mut s);                         //s is valid

    println!("{}", s); //s is valid


    let _reference1 = &mut s; //s is not valid, reference1 is valid
    //let reference2 = &mut s; //s is not valid, reference2 is not valid

    //println!( "{}, {}", reference1, reference2); //reference1 and reference2 are valid

    let reference3 = &s; //s is valid, reference3 is valid
    let reference4 = &s; //s is valid, reference4 is valid

    println!("{}, {}", reference3, reference4); //reference3 and reference4 are valid




    //the slice type

    let s = String::from("Hola mundo");

    let _slice1 = &s[0..4]; //slice1 is valid
    let _slice2 = &s[..4];  //slice2 is valid
    let _slice3 = &s[4..];  //slice3 is valid
    let _slice4 = &s[..];   //slice4 is valid

    println!(" slice1: {} \n slice2: {} \n slice3: {} \n slice4: {}", _slice1, _slice2, _slice3, _slice4); //slice1, slice2, slice3 and slice4 are valid

    let s = String::from("hello world");

    let word = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {}", word);


    //dereference operator

    let x = 5;
    let y = &x;

    if *y == 5 {
        println!("y is 5");
    }


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", soy un string");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}