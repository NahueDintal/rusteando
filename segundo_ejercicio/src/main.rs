use std::io;

fn main() {
    let mut primer_numero = String::new();
    let mut operacion = String::new();
    let mut segundo_numero = String::new();

    println!("Por favor, ingresa el primer número:");
    io::stdin()
        .read_line(&mut primer_numero)
        .expect("Error al leer el primer número");

    let primer_numero: f64 = primer_numero.trim().parse()
        .expect("Por favor, ingresa un número válido");

    println!("¿Qué operación deseas realizar? (+, -, *, /):");
    io::stdin()
        .read_line(&mut operacion)
        .expect("Error al leer la operación");

    println!("Por favor, ingresa el segundo número:");
    io::stdin()
        .read_line(&mut segundo_numero)
        .expect("Error al leer el segundo número");

    let segundo_numero: f64 = segundo_numero.trim().parse()
        .expect("Por favor, ingresa un número válido");

    // Aquí está la corrección en la estructura del match
    let resultado = match operacion.trim() {
        "+" => primer_numero + segundo_numero,
        "-" => primer_numero - segundo_numero,
        "*" => primer_numero * segundo_numero,
        "/" => {
            if segundo_numero == 0.0 {
                println!("Error: No se puede dividir por cero");
                return;
            }
            primer_numero / segundo_numero  // No necesita coma aquí
        }
        _ => {
            println!("Error: Operación no válida");
            return;
        }
    };  // El match termina aquí con punto y coma

    println!("{} {} {} = {:.2}", 
        primer_numero, 
        operacion.trim(), 
        segundo_numero, 
        resultado
    );
}
