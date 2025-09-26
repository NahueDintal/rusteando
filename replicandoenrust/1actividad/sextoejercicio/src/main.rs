use std::io;
use std::io::Write;
use std::error::Error;

fn main () -> Result<(), Box<dyn Error>> {

    let horas_trabajadas = lectura_con_reintento(":: Cant. horas trabajadas: ");
    let precio_horatrabajo = lectura_con_reintento(":: Precio de horas trabajadas: ");

    let (horas_extras, bono_horas_extras) = if horas_trabajadas >= 160.0 {
        let extras = horas_trabajadas - 160.0;
        let bono = extras * precio_horatrabajo;
        (extras, bono)
    } else {
        (0.0, 0.0)
    };

    println!(":: Horas trabajadas: {}", horas_trabajadas);
    println!(":: La cantidad de horas extras son: {}", horas_extras);
    println!(":: La cantidad de horas extras suman: ${}", bono_horas_extras);

    let sueldo_base = horas_trabajadas * precio_horatrabajo;
    let sueldo_total = sueldo_base + bono_horas_extras;

    println!(":: El sueldo total es: ${}", sueldo_total);

    Ok(())
}
fn leer_numero(mensaje: &str) -> Result<f64, Box<dyn Error>> {
    print!("{}", mensaje);
    std::io::stdout().flush()?; // ← Necesario para print!

    let mut input = String::new();
    io::stdin().read_line(&mut input)?; 

    let numero: f64 = input.trim().parse()?;
    Ok(numero)
}
fn lectura_con_reintento(mensaje: &str) -> f64 {
    loop {
        match leer_numero(mensaje) {
            Ok(num) => return num,
            Err(e) => println!(":: Error:{}", e)
        }
    }
}
