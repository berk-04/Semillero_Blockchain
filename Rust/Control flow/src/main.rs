fn main() {
    
    //if in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    //loops

    //loop in a let statement
    let mut counter = 0;

    let result = loop
    {
        if counter == 10
        {
            break counter * 2;
        }
        counter += 1;
    };

    println!("The value of result is: {:?}", result);

    //loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
    
            count += 1;
        }
        println!("End count = {count}");

    //while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //looping through a collection with for

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //range in for loop with rev method to reverse the range

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    //for loop with enumerate method to return a tuple

    let a = [4, 3, 2, 1];
    
    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("The {}th element is {}",i+1,v);
    }
}