use std::io;
use std::error::Error;

fn leer_numero(mensaje: &str) -> Result<f64, Box<dyn Error>> {
    println!("{}", mensaje);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let numero: f64 = input.trim().parse()?;
    Ok(numero)
}
fn main () -> Result<(), Box<dyn Error>> {

    let base = loop {

        match leer_numero(":: Ingresar base:") {
            Ok(num) => break num,
            Err(e) => println!(":: Error: {}", e),
        }
    };
    let altura = loop {

        match leer_numero(":: Ingresar altura:") {
            Ok(num) => break num,
            Err(e) => println!(":: Error: {}", e),
        }
    };

    println!(":: Base: {}", base);
    println!(":: Altura: {}", altura);

    let area = base * altura;
    let perimetro = 2.0 * (base + altura);


    println!(":: Area: {:.2}", area);
    println!(":: Perimetro: {:.2}", perimetro);

    Ok(())
} 
