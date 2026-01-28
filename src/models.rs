use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paciente {
    pub id: Uuid,
    pub nombre: String,
    pub edad: u8,
    pub email: String
}

#[derive(Debug, Deserialize)]
pub struct CrearPaciente {
    pub nombre: String,
    pub edad: u8,
    pub email: String
}
