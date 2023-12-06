#[cfg(test)]

mod tests{

/*
    Fix the warning below with :
    🌟 Only one solution
    🌟🌟 Two distinct solutions


    fn main() {
        let x = 1; 
    }

    // Warning: unused variable: `x`

*/

    #[test]
    fn test(){
        let _x = 1;
    }

    #[test]
    #[allow(unused_variables)]
    fn test_1(){
        let x = 1;
    }

}