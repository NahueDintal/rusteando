// Simulación de un cajero
// ingresar por consola el usuario y la clave
// user , 1234
// si el logeo es correcto dar bienvenida al sistema
// caso contrario, logeo incorrecto
// una vez ingresado el usuario tiene 500k
// opcion 1 deposito van a ingresar el monto a ingresar
// opcion 2 transferencia, solo puede transferir lo que tiene
// en ambos casos imprimir los montos de la operacion
// Entradas

use std::io;
use std::io::Write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Hello, world!");

    let usuario = "Marcelo";
    let clave = "123456";

    loop{
        if usuario && clave {
            "Bienvenido"
        } else {
            "Logeo incorrecto!"
        }

    }

    Ok(())
}
fn leer_numero(mensaje: &str) -> Result<f64, Box<dyn Error>> {
    print!("{}", mensaje);
    std::io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let numero: f64 = input.trim().parse()?;
    Ok(numero)
}
fn lectura_con_reintento(mensaje: &str) -> f64 {
    loop{
        match leer_numero(mensaje) {
            Ok(num) => return num,
            Err(e) => println!(":: Error: {}", e)
        }
    }
}
