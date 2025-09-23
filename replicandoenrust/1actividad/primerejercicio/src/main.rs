use std::io;
use std::error::Error;

fn leer_numero(mensaje: &str) -> Result<f64, Box<dyn Error>> {
    println!("{}", mensaje);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let numero: f64 = input.trim().parse()?;
    Ok(numero)
}
fn main() -> Result<(), Box<dyn Error>> {
    println!(":: Calculadora de promedio");
    println!(":: Ingrese 4 números:");

    let mut numeros = Vec::new();

    for i in 1..=4 {
        let mensaje = format!(":: Números {}:", i);
        loop {
            match leer_numero(&mensaje) {
                Ok(num) => {
                    numeros.push(num);
                    break;
                },
                Err(e) => {
                    println!("Error: {}. Por favor, ingrese un número válido.", e);
                }
            }
        }
    }
    let suma: f64 = numeros.iter().sum();
    let promedio = suma / numeros.len() as f64;

    println!(":: Números ingresados: {:?}", numeros);
    println!(":: Suma: {}", suma);
    println!(":: Promedio: {:.2}", promedio);

    Ok(())
}
