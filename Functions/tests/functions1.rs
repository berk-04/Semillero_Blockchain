fn call_me(){
    println!("Hello World!");
}

#[cfg(test)]
mod tests {

    use super::call_me;

    #[test]
    fn call_function() {
        call_me();
    }

}