use std::{io::{self, White}, thread, time::Duration};
use rpassword::read_password;

fn main() {
    // Constantes para configurar el comportamiento del programa
    const MAX_INTENTOS: u32 = 5 //Numero max de intentos permitidos
    const TIEMPO_ESPERA: u64 = 5 //segundos de espera entre intentos

    let clavesiña = String::from("QuetepasaenTurco!");
    let mut intentos = 0;
    while intentos < MAX_INTENTOS {
        let intentos_restantes = MAX_INTENTOS - intentos;
        println!("Por favor, ingrese la contraseña({} intentos restantes): ", intentos_restantes);
        
        io::stdout().flush().expect("Error al leer la contraseña!!");

        if clave == clavesiña {
            println!("\n contraseña correcta! Bienvenido al sistema!");
            return;
        } else {
            intentos += 1;

            if intentos < MAX_INTENTOS {
                println!("\n contraseña incorrecta!");
                println!("Espere {} segundos antes del siguiente intento...", TIEMPO_ESPERA);

                thread::sleep(Duration::from_secs(TIEMPO_ESPERA));

            }
        }
    }
    println!("Se han agotado todos los intentos. El programa procede a cerrarce");
}
