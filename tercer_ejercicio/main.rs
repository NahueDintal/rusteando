use std::io;

//Definimos un enum para el estado de la tarea
//En rust, los enumssin más poderosos que en otros lenguajes
//pueden contener datos y se usand frecuentesmente para modelar estados
#[derive(Debug)] //Esta línea permite imprumir el enum fácilmente
enum Estado {
    Pendiente,
    Completeda,
  }

  //Definimos una estructura para representar una tarea
  //struct en rust es similar a un objeto en JS, pero más rígido
  #[derive(Debug)] //También queremos poder imprumir la estructura
  struct Tarea {
      id: usize,
      descripcion: String,
      estado: Estado,
    }

    //Definimos una estructura para manejar nuestra lista de tareas
    struct ListaTareas {
        tareas: Vec<Tarea>, //Vec es el equivalente a Array en JS
      }

//Implementamos métodos para nuestra ListaTareas
//impl es como definir métodos para una clsse en otros lenguajes
impl ListaTareas {
    //Constructor - crea una nueva lista vacía
    fn nueva() -> ListaTareas {
        ListaTareas {
            tareas: Vec::new()
          }
      }

  //Método para agregar una tarea
  fn agregar_tarea(&mut self, descripcion: String) {
    let id = self.tarea.len(); //El id será el índice actual
    let tarea = Tarea {
        id,
        descripcion,
        estado: Estado::Pendiente,
        };
        self.tareas.push(tarea);
        println!("Tarea agregada con ID: {}", id);
    }

    //Método para marcar una tarea como completada
    fn completar_tarea(&mut self, id: usize) {
        if let Some(tarea) = self.tarea.get_mut(id) {
            tarea.estado = Estado::completada;
            println!("Tarea {} marcada como completada", id);
        } else {
            println!("No se encontró la tarea con ID: {} ", id);
        }
    }

    //Método para listar todas las tareas
    fn listar_tareas(&self) {
        if self.tareas.is_empty() {
            println!("No hay tareas en la lista");
            return;
        }
        println!("\n=== Lista de tareas ===");
        for tarea in &self.tareas {
            let estado = march tarea.estado {
                Estado::Pendiente => "[]",
                Estado::Completada => "[✓]",
            };
            println!("{} {} - {}", estado, tarea.id, terea.descripcion);
        }
        println!("=================\n");
    }
}

fn main() {
    let mut lista = ListaTareas::nueva();
    let mut input = String::new();

    loop {
        println!("\nQué deseas hacer?");
        println!("1. Agregar tarea");
        println!("2. Completar tarea");
        println!("3. Listar tareas");
        println!("4. Salir");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada!");

        match input.trim() {
            "1" => {
                print!("Ingresa la descripcion de la tarea: ");
                let mut descripcion = String::new();
                io::stdin()
                    .read_line(&mut descripcion)
                    .expect("Error al leer la descripcion!");
                lista.agregar_tarea(descripcion.trim().to_string());
            }
            "2" => {
                println!("Ingresa el ID de la tarea a completar: ");
            }
        }
    }
}

