pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables

    match a > b {
        true => a,
        false => b,
    }
}

pub fn foo_if_fizz(fizzish: &str) -> &str {
    match fizzish {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "baz",
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }

    #[test]
    fn test_primitive_types1() {
        let is_morning = true;
        if is_morning {
            println!("Good morning!");
        }
    
        let is_evening = true;      // Finish the rest of this line like the example! Or make it be false!
        if is_evening {
            println!("Good evening!");
        }    
    }

    #[test]
    fn test_primitive_types2() {

        // Characters (`char`)
        // Note the _single_ quotes, these are different from the double quotes
        // you've been seeing around.
        
        let my_first_initial = 'C';
        if my_first_initial.is_alphabetic() {
            println!("Alphabetical!");
        } else if my_first_initial.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }

        // Finish this line like the example! What's your favorite character?
        // Try a letter, try a number, try a special character, try a character
        // from a different language than your own, try an emoji!

        let your_character: char = 'K';

        if your_character.is_alphabetic() {
            println!("Alphabetical!");
        } else if your_character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }


        let your_character: char = '7';

        if your_character.is_alphabetic() {
            println!("Alphabetical!");
        } else if your_character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }
        

        let your_character: char = '°';

        if your_character.is_alphabetic() {
            println!("Alphabetical!");
        } else if your_character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }


        let your_character: char = '😃';

        if your_character.is_alphabetic() {
            println!("Alphabetical!");
        } else if your_character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }
    }

    #[test]
    fn test_primitive_types3() {
        let a: [u8; 77] = [0; 77];

        if a.len() >= 100 {
            println!("Wow, that's a big array!");
        } else {
            println!("Meh, I eat arrays like that for breakfast.");
        }        
    }

/*

// Fill in the blanks
fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } __ n > 0 {
        println!("{} is positive", n);
    } __ {
        println!("{} is zero", n);
    }
} 

*/

    #[test]
    fn test() {
        let n = 5;

        if n < 0 {
            println!("{} is negative", n);
        } else if n > 0 {
            println!("{} is positive", n);
        } else {
            println!("{} is zero", n);
        }
    }

/*

// Fix the errors
fn main() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 ;
        }

    println!("{} -> {}", n, big_n);
} 

*/

    #[test]
    fn test_1() {
        let n = 5;

        let big_n =
            if n < 10 && n > -10 {
                println!(", and is a small number, increase ten-fold");
                10 * n
            } else {
                println!(", and is a big number, halve the number");
                n / 2
            };

        println!("{n} -> {big_n}");
    } 

/*

*/

}