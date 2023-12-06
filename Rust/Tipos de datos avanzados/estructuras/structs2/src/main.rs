#[derive(Debug)]
struct Carro {
    marca: String,
    modelo: String,
    placa: str,
}

struct Valores(f32, i64);

#[derive(Debug)]
struct Direccion {
    calle: String,
    numero: u64,
    provincia: String,
    codigo_postal: String,
}

#[derive(Debug)]
struct Persona {
    apellidos: String,
    nombres: String,
    edad: u16,
    direccion: Direccion,
}



fn main() {
  let p = Persona {
      apellidos: String::from("SERNA"),
      nombres: String::from("FABIAN"),
      edad: 30_u16,
      direccion: Direccion {
          calle: String::from("Carrera 2 N"),
          numero: 18,
          provincia: String::from("Dosquebradas"),
          codigo_postal: String::from("110002"),
      },
  };

  println!("{:?}", p);

  let v = Valores(10.5_f32, 18_i64);
  println!("{} {}", v.0, v.1);
}