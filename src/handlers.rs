use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::{Paciente, CrearPaciente};
use crate::state::AppState;

pub async fn crear_paciente(
    data: web::Data<AppState>,
    body: web::Json<CrearPaciente>,
) -> impl Responder {
    let mut pacientes = data.pacientes.lock().unwrap();

    let paciente = Paciente {
        id: Uuid::new_v4(),
        name: body.name.clone(),
        edad: body.edad,
        email: body.email.clone()
    };

    pacientes.push(paciente.clone());
    HttpResponse::Created().json(paciente)
}

pub async fn actualizar_paciente(
    data: web::Data<AppState>,
    paciente_id: web::Path<Uuid>,
    body: web::Json<CrearPaciente>
) -> impl Responder {
    let mut pacientes = data.pacientes.lock().unwrap();

    match pacientes.iter_mut().find(|paciente| paciente.id == * paciente_id) {
        Some(paciente) => {
            paciente.name = body.name.clone();
            paciente.edad = body.edad;
            paciente.email = body.email.clone();
            HttpResponse::Ok().json(paciente)
        }
        None => { 
            HttpResponse::NotFound().json("Paciente no encontrado")
        }
    }
}

pub async fn listar_paciente(data: web::Data<AppState>) -> impl Responder {
    let pacientes = data.pacientes.lock().unwrap();
    HttpResponse::Ok().json(&*pacientes)
}

pub async fn obtener_paciente(
    data: web::Data<AppState>,
    paciente_id: web::Path<Uuid>
) -> impl Responder {
    let pacientes = data.pacientes.lock().unwrap();
    println!("{paciente_id}");
    match pacientes.iter().find(|paciente| paciente.id == *paciente_id) {
        Some(paciente) => HttpResponse::Ok().json(paciente),
        None => HttpResponse::NotFound().body("Paciente no encontrado")
    }
}

pub async fn eliminar_paciente(
    data: web::Data<AppState>,
    paciente_id: web::Path<Uuid>
) -> impl Responder {
    let mut pacientes = data.pacientes.lock().unwrap();
    let len_inicial = pacientes.len();

    pacientes.retain(|paciente| paciente.id != *paciente_id);

    if pacientes.len() == len_inicial {
        HttpResponse::NotFound().body("Paciente no encontrado")
    } else {
        HttpResponse::Ok().body("Paciente eliminado")
    }
}
