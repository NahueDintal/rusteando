mod models;
mod storage;
mod error;

pub use models::*;
pub use storage::*;
pub use error::*;

pub struct GestorClientes {
    clientes: Vec<Cliente>,
    next_id: u32,
}

impl GestorClientes {
    pub fn new() -> Result<Self, AppError> {
        let clientes = cargar_datos()?;
        let next_id = clientes.iter().map(|c| c.id).max().unwrap_or(0) + 1;
        Ok(Self { clientes, next_id })
    }
    
    pub fn agregar_cliente(&mut self, nombre: String, contacto: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        
        let cliente = Cliente {
            id,
            nombre,
            contacto,
            equipos: Vec::new(),
        };
        
        self.clientes.push(cliente);
        id
    }
    
    pub fn agregar_equipo(
        &mut self,
        cliente_id: u32,
        tipo: TipoEquipo,
        modelo: String
    ) -> Result<u32, AppError> {
        let cliente = self.clientes.iter_mut()
            .find(|c| c.id == cliente_id)
            .ok_or(AppError::ClienteNoEncontrado)?;
        
        let equipo_id = cliente.equipos.iter()
            .map(|e| e.id)
            .max()
            .unwrap_or(0) + 1;
            
        cliente.equipos.push(Equipo {
            id: equipo_id,
            tipo,
            modelo,
            historial: Vec::new(),
        });
        
        Ok(equipo_id)
    }
    
    pub fn registrar_mantenimiento(
        &mut self,
        cliente_id: u32,
        equipo_id: u32,
        tecnico: String,
        descripcion: String,
        repuestos: Vec<String>
    ) -> Result<(), AppError> {
        let mantenimiento_id = Utc::now().timestamp() as u32;
        
        let cliente = self.clientes.iter_mut()
            .find(|c| c.id == cliente_id)
            .ok_or(AppError::ClienteNoEncontrado)?;
            
        let equipo = cliente.equipos.iter_mut()
            .find(|e| e.id == equipo_id)
            .ok_or(AppError::EquipoNoEncontrado)?;
            
        equipo.historial.push(Mantenimiento {
            id: mantenimiento_id,
            fecha: Utc::now(),
            tecnico,
            descripcion,
            repuestos,
        });
        
        Ok(())
    }
    
    pub fn guardar_cambios(&self) -> Result<(), AppError> {
        guardar_datos(&self.clientes)
    }
}
