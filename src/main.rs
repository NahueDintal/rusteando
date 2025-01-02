// Acá empezando con el Rustito!
use std::collections::{HashSet, HashMap};

fn main() {
    println!("Hola Rustito!!");

//Comentario 
/* aca otro comentario
 y acá más comentario */

// Variables

let my_string: &str = "Esto es una cadena de testo";
println!("{my_string}");
let my_string: &str = "Aquí cambio el testo";
println!("{my_string}");

let my_string2: String = String::from("Es otra forma de hacer una variable string");
println!("{my_string2}");

let mut my_int: i32 = 7;
my_int = my_int + 4;
println!("{my_int}");
println!("{}", my_int -1);
println!("Este es el valor de mi int: {}", my_int);

let my_float: f64 = 6.5;
println!("Este es un float: {my_float}");
// my_float = my_float + my_int: Error
// Por que uno es i32 y el otro es un f64

let my_buleano: bool = false;
println!("{my_buleano}");

// Constantes
const MY_CONSTANTE: &str = "Mi primer constante global";
println!("{MY_CONSTANTE}");

// Constrol de flujo
if my_int == 10 {
    println!("El valor es 10");
} else if my_int == 11{
    println!("El valor es 11");
} else {
    println!("El valor no es ni 10 ni 11");
}

//Lista
let mut mi_lista: Vec<&str> = vec!["nahue", "dintal", "@nahue.dintal"];
mi_lista.push("edad 36");
println!("{:?}", mi_lista);

// Set
let mut mi_set: HashSet<&str> = vec!["nahue", "dintal", "@nahue.dintal"].into_iter().collect();
mi_set.insert("nahue");
println!("{:?}", mi_set);

// Set
let mut mi_mapa: HashMap<&str, i32> = vec![("nahue", 36),("cata", 11),("pipita", 7)]
    .into_iter()
    .collect();
mi_mapa.insert("florenchia", 32);
println!("{:?}", mi_mapa);

//Bucles
for value: &str in mi_lista {
    println!("{}", value)
}


}
 
