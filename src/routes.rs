use actix_web::{web};
use crate::handlers::{crear_paciente, listar_paciente, obtener_paciente, actualizar_paciente, eliminar_paciente};

pub fn pacientes_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pacientes")
            .route("", web::post().to(crear_paciente))
            .route("", web::get().to(listar_paciente))
            .route("/{id}", web::get().to(obtener_paciente))
            .route("/{id}", web::patch().to(actualizar_paciente))
            .route("/{id}", web::delete().to(eliminar_paciente))
    );
}
