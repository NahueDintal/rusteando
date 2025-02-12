use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de IO: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Error de serialización: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("Cliente no encontrado")]
    ClienteNoEncontrado,
    
    #[error("Equipo no encontrado")]
    EquipoNoEncontrado,
    
    #[error("ID duplicado: {0}")]
    IdDuplicado(u32),
}
