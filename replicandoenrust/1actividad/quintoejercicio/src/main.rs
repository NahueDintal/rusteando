use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { 

    let pesos = lectura_con_reintento(":: Ingresar cantidad de pesos ");

    let dolares = 1450.0 * pesos;
    let euros = 1200.0 * pesos;
    let reales = 700.0 * pesos;

    println!(":: Valor de Dólares: u$s {}", dolares);
    println!(":: Valor de Euros: {}", euros);
    println!(":: Valor de Reales: {}", reales);

    Ok(())
}
fn leer_numero(mensaje: &str) -> Result<f64, Box<dyn Error>> {
     println!("{}", mensaje);

     let mut input = String::new();
     io::stdin().read_line(&mut input)?;

     let numero: f64 = input.trim().parse()?;
     Ok(numero)
}
fn lectura_con_reintento(mensaje: &str) -> f64 {
    loop {
        match leer_numero(mensaje) {
            Ok(num) => return num,
            Err(e) => println!(":: Error: {}", e)
        }
    }
}
