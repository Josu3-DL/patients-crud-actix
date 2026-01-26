use std::sync::Mutex;
use crate::models::Paciente;

pub struct AppState {
    pub pacientes: Mutex<Vec<Paciente>>
}
