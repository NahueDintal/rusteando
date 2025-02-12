use std::io;
use std::thread;
use std::time::Duration;
use rpassword::read_password;

fn main() {
    // Constantes para configurar el comportamiento del programa
    const MAX_INTENTOS: u32 = 5; // Número máximo de intentos permitidos
    const TIEMPO_ESPERA: u64 = 5; // Segundos de espera entre intentos

    let clavesiña = "QuetepasaenTurco!";
    let mut intentos = 0;

    while intentos < MAX_INTENTOS {
        let intentos_restantes = MAX_INTENTOS - intentos;
        println!("Por favor, ingrese la contraseña ({} intentos restantes): ", intentos_restantes);

        let mut clave = read_password().expect("Error al leer la línea!!");

        if clave.trim() == clavesiña {
            println!("\n¡Contraseña correcta! ¡Bienvenido al sistema!");
            return;
        } else {
            intentos += 1;
            if intentos < MAX_INTENTOS {
                println!("\n¡Contraseña incorrecta!");
                println!("Espere {} segundos antes del siguiente intento...", TIEMPO_ESPERA);
                thread::sleep(Duration::from_secs(TIEMPO_ESPERA));
            }
        }
    }

    println!("¡Has excedido el número máximo de intentos!");
}
