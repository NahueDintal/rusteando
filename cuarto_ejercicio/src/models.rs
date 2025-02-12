use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub contacto: String,
    pub equipos: Vec<Equipo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipo {
    pub id: u32,
    pub tipo: TipoEquipo,
    pub modelo: String,
    pub historial: Vec<Mantenimiento>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mantenimiento {
    pub id: u32,
    pub fecha: DateTime<Utc>,
    pub tecnico: String,
    pub descripcion: String,
    pub repuestos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoEquipo {
    Computadora,
    Impresora,
    Servidor,
    Red,
    Otro(String),
}
