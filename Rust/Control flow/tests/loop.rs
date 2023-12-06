#[cfg(test)]

mod tests{

/*

fn main() {
    for n in 1..=100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 

*/

    #[test]
    fn test() {
        for n in 1..100 { // modify this line to make the code work
            if n == 100 {
                panic!("NEVER LET THIS RUN")
            }
        }

        println!("Success!");
    } 

/*

// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
    }
    
    println!("{:?}", numbers);
} 

*/
    #[test]
    fn test_1() {
        let names = [String::from("liming"),String::from("hanmeimei")];
        for name in &names {
            println!("{}", name);
        }

        println!("{:?}", names);

        let numbers = [1, 2, 3];
        // The elements in numbers are Copy，so there is no move here
        for n in numbers {
            println!("{}", n);
        }
        
        println!("{:?}", numbers);
    } 

/*
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}
*/

    #[test]
    fn test_2() {
        let a = [4, 3, 2, 1];
    
        // Iterate the indexing and value in 'a'
        for (i,v) in a.iter().enumerate() {
            println!("The {}th element is {}",i+1,v);
        }
    }

/*


// Fill in the blanks
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            __;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            __;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

*/

    #[test]
    fn test_3() {
        let mut count = 0u32;

        println!("Let's count until infinity!");

        // Infinite loop
        loop {
            count += 1;

            if count == 3 {
                println!("three");

                // Skip the rest of this iteration
                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("OK, that's enough");

                break;
            }
        }

        assert_eq!(count, 5);

        println!("Success!");
    }

/*

// Fill in the blank
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            __;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

*/

    #[test]
    fn test_4() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);

        println!("Success!");
    }

/*

// Fill in the blank
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == __);

    println!("Success!");
}

*/

    #[test]
    fn test_5() {
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    // This would break only the inner1 loop
                    break 'inner1; // `break` is also works.
                }
                count += 2;
            }

            count += 5;

            loop {
                if count >= 30 {
                    // This breaks the outer loop
                    break 'outer;
                }

                // This will continue the outer loop
                continue 'outer;
            }
        }

        assert!(count == 30);

        println!("Success!");
    }


}