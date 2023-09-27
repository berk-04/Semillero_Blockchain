mod my {
    #[derive(Debug)]
    pub struct Persona {
        pub apellidos : String,
        pub nombres: String,
        pub edad: u32
    }


    #[allow(dead_code)]
    impl Persona {
        pub fn imprimir(&self) {
            println!("{:?}",self);
        }
    }    

}

#[derive(Debug)]
pub struct Persona {
    apellidos : String,
    nombres: String,
    edad: u32
}

fn main() {
    let persona = Persona { apellidos : String::from("VALERA"),
        nombres : String::from("RAMON"),
        edad : 30_u32
    };
//    persona.imprimir();
   println!("{:?}",persona);
    
}