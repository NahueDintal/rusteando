use std::io;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
enum Estado {
    Pendiente,
    Completada,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tarea {
    id: usize,
    descripcion: String,
    estado: Estado,
}

struct ListaTareas {
    tareas: Vec<Tarea>,
}

impl ListaTareas {
    fn cargar() -> Self {
        match fs::read_to_string("tareas.json") {
            Ok(contenido) => {
                let tareas: Vec<Tarea> = serde_json::from_str(&contenido)
                    .unwrap_or_else(|_| {
                        eprintln!("Error al parsear el archivo, creando nueva lista");
                        Vec::new()
                    });
                ListaTareas { tareas }
            }
            Err(_) => {
                println!("No se encontró archivo, iniciando lista nueva");
                ListaTareas { tareas: Vec::new() }
            }
        }
    }

    fn guardar(&self) {
        let contenido = serde_json::to_string_pretty(&self.tareas)
            .expect("Error al serializar las tareas");
        
        fs::write("tareas.json", contenido)
            .expect("Error al escribir en el archivo");
        
        println!("¡Datos guardados correctamente!");
    }

     fn agregar_tarea(&mut self, descripcion: String) {
        let id = self.tareas.len();
        self.tareas.push(Tarea {
            id,
            descripcion,
            estado: Estado::Pendiente,
        });
        self.guardar();  // Guardamos después de cada operación
        println!("✓ Tarea agregada con ID: {}", id);
    }

    fn completar_tarea(&mut self, id: usize) {
        if let Some(tarea) = self.tareas.get_mut(id) {
            tarea.estado = Estado::Completada;
            self.guardar();  // Guardamos después de cada operación
            println!("✓ Tarea {} marcada como completada", id);
        } else {
            println!("× No existe la tarea con ID: {}", id);
        }
    }

    fn listar_tareas(&self) {
        if self.tareas.is_empty() {
            println!("La lista de tareas está vacía");
            return;
        }

        println!("\n=== Lista de Tareas ===");
        for tarea in &self.tareas {
            let estado = match tarea.estado {
                Estado::Pendiente => " ",
                Estado::Completada => "✓",
            };
            println!("[{}] {} - {}", estado, tarea.id, tarea.descripcion);
        }
        println!("======================\n");
    }
}

fn main() {
    let mut lista = ListaTareas::cargar();
    
    loop {
        println!("1. ➕ Agregar tarea");
        println!("2. ✅ Completar tarea");
        println!("3. 📋 Listar tareas");
        println!("4. 🚪 Salir");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error leyendo entrada");

        match opcion.trim() {
            "1" => {
                println!("Descripción de la tarea:");
                let mut desc = String::new();
                io::stdin()
                    .read_line(&mut desc)
                    .expect("Error leyendo descripción");
                lista.agregar_tarea(desc.trim().to_string());
            }
            "2" => {
                println!("ID de la tarea a completar:");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Error leyendo ID");
                match id.trim().parse() {
                    Ok(id) => lista.completar_tarea(id),
                    Err(_) => println!("× ID inválido"),
                }
            }
            "3" => lista.listar_tareas(),
            "4" => {
                lista.guardar();
                println!("¡Hasta luego!");
                break;
            }
            _ => println!("× Opción no válida"),
        }
    }
}
