/*transformar la estructura Container que solo acepta enteros positivos del tipo u32 en un contenedor 
genérico que puede contener valores de cualquier tipo determinado.
 Este ejercicio se completa cuando se compila el código */

struct Container<U>{
    value: U,
}

impl<U>  Container <U> {
    pub fn new(value: U) -> Self {
        Container { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}