// hacer una calculadora de primedio
// donde se ingresan las notas 
// y se calcula el promedio de la mismas

use std::io;

fn main() {
    println!("Ingrese numeros para calcular el promedio.");
    println!("Numero 1");
    let mut numero1 = String::new();
    io::stdin().read_line(&mut numero1).expect("Error");
    println!("Numero 2");
    let mut numero2 = String::new();
    io::stdin().read_line(&mut numero2).expect("Error");
    println!("Numero 3");
    let mut numero3 = String::new();
    io::stdin().read_line(&mut numero3).expect("Error");
    println!("Numero 4");
    let mut numero4 = String::new();
    io::stdin().read_line(&mut numero4).expect("Error");

    let numero1 = convertirNumero(&numero1);

    println!("Numero 1: {:?}", numero1);
    println!("Numero 2: {}", numero2);
    println!("Numero 3: {}", numero3);
    println!("Numero 4: {}", numero4);
}
fn convertirNumero(texto: &str) -> Result<f64, String> {
    match texto.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(e) => Err(format!("Error: {}", e))
    }
}
