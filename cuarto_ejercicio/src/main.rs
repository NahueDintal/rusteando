use clientes_equipos::*;

fn main() -> Result<(), AppError> {
    let mut gestor = GestorClientes::new()?;
    
    loop {
        println!("\n=== Gestión de Clientes y Equipos ===");
        println!("1. Nuevo Cliente");
        println!("2. Agregar Equipo");
        println!("3. Registrar Mantenimiento");
        println!("4. Listar Todos");
        println!("5. Salir y Guardar");
        
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion)?;
        
        match opcion.trim() {
            "1" => crear_cliente(&mut gestor)?,
            "2" => agregar_equipo(&mut gestor)?,
            "3" => registrar_mantenimiento(&mut gestor)?,
            "4" => listar_todo(&gestor)?,
            "5" => {
                gestor.guardar_cambios()?;
                println!("¡Datos guardados correctamente!");
                break;
            }
            _ => println!("Opción inválida"),
        }
    }
    
    Ok(())
}

// Funciones auxiliares para cada operación
fn crear_cliente(gestor: &mut GestorClientes) -> Result<(), AppError> {
    println!("Nombre del cliente:");
    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre)?;
    
    println!("Contacto:");
    let mut contacto = String::new();
    io::stdin().read_line(&mut contacto)?;
    
    let id = gestor.agregar_cliente(nombre.trim().to_string(), contacto.trim().to_string());
    println!("Cliente creado con ID: {}", id);
    Ok(())
}

// Implementar funciones restantes de manera similar...
