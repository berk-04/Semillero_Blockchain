#[derive(Debug)]
struct Direccion<'b> {
    calle: &'b mut String,
    numero: &'b mut u64,
    provincia: &'b mut String,
    codigo_postal: &'b mut String,
}

#[derive(Debug)]
struct Persona<'a> {
    apellidos: &'a mut String,
    nombres: &'a mut String,
    edad: &'a mut u16,
    direccion: &'a mut Direccion<'a>,
}

fn main() {
    let p = Persona {
      apellidos: &mut String::from("VASQUEZ"),
      nombres: &mut String::from("CAMILO"),
      edad: &mut 30_u16,
      direccion: &mut Direccion {
          calle: &mut String::from("calle 18"),
          numero: &mut 1,
          provincia: &mut String::from("dos"),
          codigo_postal: &mut String::from("11001"),
      },
  };

  
    p.apellidos.push_str(" ARANGUREN");
    let mut codigo_postal2 = String::from("18015");
    p.direccion.codigo_postal = &mut codigo_postal2;
    println!("{:?}", p);
  

  p.direccion.codigo_postal.clear();
  p.direccion.codigo_postal.push_str("18018");
  println!("{:?}", p);  

}
