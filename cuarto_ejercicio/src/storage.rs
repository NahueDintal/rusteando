use crate::error::AppError;
use std::fs::{File, OpenOptions};
use std::path::Path;

const DB_FILE: &str = "clientes_db.json";

pub fn guardar_datos(clientes: &[Cliente]) -> Result<(), AppError> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(DB_FILE)?;
    
    serde_json::to_writer_pretty(file, clientes)?;
    Ok(())
}

pub fn cargar_datos() -> Result<Vec<Cliente>, AppError> {
    if !Path::new(DB_FILE).exists() {
        return Ok(Vec::new());
    }
    
    let file = File::open(DB_FILE)?;
    let clientes = serde_json::from_reader(file)?;
    Ok(clientes)
}
