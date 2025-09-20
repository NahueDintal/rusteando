use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: tasker <comando> [opciones]");
        return Ok(());
    }

    let comando = &args[1];

    match comando.as_str() {
        "add" => {
            // Inicializar variables para los datos de la tarea
            let mut nombre = None;
            let mut fecha = None;
            let mut descripcion = None;
            let mut tipo = None;
            let mut prioridad = None;

            // Iterar sobre los argumentos
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "-n" => {
                        if i + 1 < args.len() {
                            nombre = Some(args[i+1].clone());
                            i += 1; // Saltar al siguiente porque ya procesamos el valor
                        }
                    },
                    "-f" => {
                        if i + 1 < args.len() {
                            fecha = Some(args[i+1].clone());
                            i += 1;
                        }
                    },
                    "-d" => {
                        if i + 1 < args.len() {
                            descripcion = Some(args[i+1].clone());
                            i += 1;
                        }
                    },
                    "-t" => {
                        if i + 1 < args.len() {
                            tipo = Some(args[i+1].clone());
                            i += 1;
                        }
                    },
                    "-p" => {
                        if i + 1 < args.len() {
                            prioridad = Some(args[i+1].clone());
                            i += 1;
                        }
                    },
                    _ => {
                        // Si no es un flag, asumimos que es parte de la descripción?
                        // O mostrar error?
                    }
                }
                i += 1;
            }

            // Ahora, abrir el archivo y escribir la tarea
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("task.txt")?;

            // Formatear la tarea como una línea en el archivo
            // Por ejemplo: nombre,fecha,descripcion,tipo,prioridad
            let linea = format!("{},{},{},{},{}\n",
                nombre.unwrap_or_default(),
                fecha.unwrap_or_default(),
                descripcion.unwrap_or_default(),
                tipo.unwrap_or_default(),
                prioridad.unwrap_or_default()
            );

            file.write_all(linea.as_bytes())?;
            println!("Tarea agregada.");
        }
        "list" => {
            // Leer el archivo y mostrar las tareas
            let contenido = std::fs::read_to_string("task.txt")?;
            for linea in contenido.lines() {
                println!("{}", linea);
            }
        }
        _ => {
            println!("Comando no reconocido: {}", comando);
        }
    }

    Ok(())
}
