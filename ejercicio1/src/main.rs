use std::io;

fn main() {
// Saludo interactivo
    let mut name = String::new();

    println!("Bienvenido a saludo interactivo, por favor ingresa tu nombre!");

    io::stdin()
        .read_line(&mut name)
        .expect("Error al leer la línea");

    if name = name.trim().chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        println!("Hola {}! Bienvenido a Arch Rust!!", name.trim;
    } else {
        println!("Porfavor ingresa solo texto!!");
    }

}
