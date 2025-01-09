use std::io;

// Definimos un enum para el estado de la tarea
// En Rust, los enums son más poderosos que en otros lenguajes
// pueden contener datos y se usan frecuentemente para modelar estados
#[derive(Debug)]  // Esta línea permite imprimir el enum fácilmente
enum Estado {
    Pendiente,
    Completada,
}

// Definimos una estructura para representar una tarea
// struct en Rust es similar a un objeto en JavaScript, pero más rígido y tipado
#[derive(Debug)]  // También queremos poder imprimir la estructura
struct Tarea {
    id: usize,           // usize es el tipo para índices, siempre positivo
    descripcion: String, // String es el tipo para texto que puede cambiar
    estado: Estado,      // Usamos nuestro enum Estado
}

// Definimos una estructura para manejar nuestra lista de tareas
struct ListaTareas {
    tareas: Vec<Tarea>,  // Vec es el equivalente a Array en JavaScript
}

// Implementamos métodos para nuestra ListaTareas
// impl es como definir métodos para una clase en otros lenguajes
impl ListaTareas {
    // Constructor - crea una nueva lista vacía
    fn nueva() -> ListaTareas {
        ListaTareas {
            tareas: Vec::new()
        }
    }

    // Método para agregar una tarea
    fn agregar_tarea(&mut self, descripcion: String) {
        let id = self.tareas.len();  // El ID será el índice actual
        let tarea = Tarea {
            id,
            descripcion,
            estado: Estado::Pendiente,
        };
        self.tareas.push(tarea);
        println!("Tarea agregada con ID: {}", id);
    }

    // Método para marcar una tarea como completada
    fn completar_tarea(&mut self, id: usize) {
        if let Some(tarea) = self.tareas.get_mut(id) {
            tarea.estado = Estado::Completada;
            println!("Tarea {} marcada como completada", id);
        } else {
            println!("No se encontró la tarea con ID: {}", id);
        }
    }

    // Método para listar todas las tareas
    fn listar_tareas(&self) {
        if self.tareas.is_empty() {
            println!("No hay tareas en la lista");
            return;
        }

        println!("\n=== Lista de Tareas ===");
        for tarea in &self.tareas {
            let estado = match tarea.estado {
                Estado::Pendiente => "[ ]",
                Estado::Completada => "[✓]",
            };
            println!("{} {} - {}", estado, tarea.id, tarea.descripcion);
        }
        println!("====================\n");
    }
}

fn main() {
    let mut lista = ListaTareas::nueva();
    let mut input = String::new();

    loop {
        println!("\n¿Qué deseas hacer?");
        println!("1. Agregar tarea");
        println!("2. Completar tarea");
        println!("3. Listar tareas");
        println!("4. Salir");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");

        match input.trim() {
            "1" => {
                println!("Ingresa la descripción de la tarea:");
                let mut descripcion = String::new();
                io::stdin()
                    .read_line(&mut descripcion)
                    .expect("Error al leer la descripción");
                lista.agregar_tarea(descripcion.trim().to_string());
            }
            "2" => {
                println!("Ingresa el ID de la tarea a completar:");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Error al leer el ID");
                if let Ok(id) = id.trim().parse() {
                    lista.completar_tarea(id);
                } else {
                    println!("ID inválido");
                }
            }
            "3" => lista.listar_tareas(),
            "4" => break,
            _ => println!("Opción inválida"),
        }
    }
}
