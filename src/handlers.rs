use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::{Paciente, CrearPaciente};
use crate::state::AppState;
use crate::errors::ApiError;

pub async fn crear_paciente(
    data: web::Data<AppState>,
    body: web::Json<CrearPaciente>,
) -> Result<impl Responder, ApiError> {
    let mut pacientes = data
        .pacientes
        .lock()
        .map_err(|_| ApiError::InternalServerError)?;

    let paciente = Paciente {
        id: Uuid::new_v4(),
        nombre: body.nombre.clone(),
        edad: body.edad,
        email: body.email.clone()
    };

    pacientes.push(paciente.clone());
    Ok(HttpResponse::Created().json(paciente))
}

pub async fn actualizar_paciente(
    data: web::Data<AppState>,
    paciente_id: web::Path<Uuid>,
    body: web::Json<CrearPaciente>
) -> Result<impl Responder, ApiError> {
    let mut pacientes = data
        .pacientes
        .lock()
        .map_err(|_| ApiError::InternalServerError)?;
    
    match pacientes.iter_mut().find(|paciente| paciente.id == * paciente_id) {
        Some(paciente) => {
            paciente.nombre = body.nombre.clone();
            paciente.edad = body.edad;
            paciente.email = body.email.clone();
            Ok(HttpResponse::Ok().json(paciente))
        }
        None => { 
            Err(ApiError::NotFound)
        }
    }
}

pub async fn listar_paciente(
    data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let pacientes = data
        .pacientes
        .lock()
        .map_err(|_| ApiError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(&*pacientes))
}

pub async fn obtener_paciente(
    data: web::Data<AppState>,
    paciente_id: web::Path<Uuid>
) -> Result<impl Responder, ApiError> {
    let pacientes = data.
        pacientes
        .lock()
        .map_err(|_| ApiError::InternalServerError)?;

    println!("{paciente_id}");
    match pacientes.iter().find(|paciente| paciente.id == *paciente_id) {
        Some(paciente) => Ok(HttpResponse::Ok().json(paciente)),
        None => Err(ApiError::NotFound)
    }
}

pub async fn eliminar_paciente(
    data: web::Data<AppState>,
    paciente_id: web::Path<Uuid>
) -> Result<impl Responder, ApiError> {
    let mut pacientes = data.pacientes.lock().unwrap();
    let len_inicial = pacientes.len();

    pacientes.retain(|paciente| paciente.id != *paciente_id);

    if pacientes.len() == len_inicial {
        Err(ApiError::NotFound)
    } else {
        Ok(HttpResponse::Ok().body("Paciente eliminado"))
    }
}
