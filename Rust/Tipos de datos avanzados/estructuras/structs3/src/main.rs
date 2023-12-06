#[derive(Debug)]
struct Persona<'a> {
    apellidos: &'a mut String,
    nombres: &'a mut String,
    edad: &'a mut u16,
}

fn main() {
    let mut p = Persona {
        apellidos: &mut String::from("VALERA"),
        nombres: &mut String::from("FABIAN"),
        edad: &mut 30_u16,
    };
    p.apellidos.push_str(" ARANGUREN");
    let mut codigo_postal = String::from("18015");
    println!("{:?}", p);

    {
        let mut val_apellidos = String::from("SERNA VILLA");
        p.apellidos =  &mut val_apellidos;
        println!("REFERENCIA A NUEVO VALOR: {:?}", p);
    }
}