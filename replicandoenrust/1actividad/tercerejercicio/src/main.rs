use std::io;
use std::error::Error;

// Ingresar por teclado el total de una venta, el procentaje
// de descuenteo y calcular el total a pagar.
// Variables

fn main() -> Result<(), Box<dyn Error>> {
    let subtotal = lectura_con_reintento(":: Ingresar subtotal");

    // el porcentaje
    
    let porcentaje = lectura_con_reintento(":: Ingresar porcentaje");

    // proceso de calculo de total
    
    let descuento = subtotal * (porcentaje / 100.0);

    let _total = subtotal - descuento;

    //Impresion 
    println!(":: Subtotal: ${}", subtotal);
    println!(":: Porcentaje: {}%", porcentaje);
    println!(":: Descuento: ${}", descuento);
    println!(":: Total: ${}", _total);

    Ok(())
}
fn lectura_con_reintento(mensaje: &str) -> f64 {
    loop {
        match leer_numero(mensaje) {
            Ok(num) => return num,
            Err(e) => println!(":: Error: {}", e),
        }
    }
}
fn leer_numero(mensaje: &str) -> Result<f64, Box<dyn Error>> {
    println!("{}", mensaje);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let numero: f64 = input.trim().parse()?;
    Ok(numero)
}
