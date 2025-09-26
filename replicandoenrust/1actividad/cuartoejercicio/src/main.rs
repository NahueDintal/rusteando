// Realizar un programa que permita ingresar la cantidad de 
// alumnos inscriptos de inglés, francés y portugués de un 
// instituto de idiomas y calcular el porcentaje que representa 
// cada uno de ellos.

use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { 

    println!(":: Ingresar cantidad alumnos por idioma");

    let alumnos_ingles = lectura_con_reintento(":: Inglés ");
    let alumnos_frances = lectura_con_reintento(":: Francés ");
    let alumnos_portuges = lectura_con_reintento(":: Portugues ");

    let suma_alumnos = alumnos_ingles + alumnos_portuges + alumnos_frances;

    let porcentaje_ingles = (alumnos_ingles * 100.0) / suma_alumnos;
    let porcentaje_frances = (alumnos_frances * 100.0) / suma_alumnos;
    let porcentaje_portuges = (alumnos_portuges * 100.0) / suma_alumnos;

    println!(":: Porcentaje de Inglés: {}%", porcentaje_ingles);
    println!(":: Porcentaje de Francés: {}%", porcentaje_frances);
    println!(":: Porcentaje de Portugues: {}%", porcentaje_portuges);
    
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
