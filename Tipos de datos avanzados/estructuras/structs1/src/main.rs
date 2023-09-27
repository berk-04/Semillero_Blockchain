#[derive(Debug)]
pub struct Persona {
    nombres: String,
    apellidos : String,
    edad: u32
}

fn main() {
    let struct_init = Persona { apellidos : String::from("SERNA"),
        nombres : String::from("RAMON"),
        edad : 35_u32
    };
   println!("{:?}",struct_init);    
}