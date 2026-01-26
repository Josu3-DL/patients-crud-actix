use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paciente {
    pub id: Uuid,
    pub name: String,
    pub edad: u8,
    pub email: String
}

#[derive(Debug, Deserialize)]
pub struct CrearPaciente {
    pub name: String,
    pub edad: u8,
    pub email: String
}
