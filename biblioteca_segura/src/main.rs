use biblioteca_segura::{save_encrypted_data, load_encrypted_data};
use std::io::{self, Write};

fn main() {
    let data = b"Información sensible de seguridad, contraseñas, dispositivos, etc.";
    let password = "supersegura";

    // Guardar datos cifrados
    save_encrypted_data("data.enc", data, password).expect("Error al guardar los datos cifrados");

    // Cargar datos cifrados
    let decrypted_data = load_encrypted_data("data.enc", password).expect("Error al descifrar los datos");

    println!("Datos descifrados: {}", String::from_utf8_lossy(&decrypted_data));
}
