use std::io;

fn main() {
    // Saludo interactivo
    let mut name = String::new();
    println!("Bienvenido a saludo interactivo, por favor ingresa tu nombre!");
    io::stdin()
        .read_line(&mut name)
        .expect("Error al leer la línea");
    let name = name.trim();
    println!("Hola {}! Bienvenido a Arch Rust!!", name);  // Nota: deberías usar name_return aquí
}
