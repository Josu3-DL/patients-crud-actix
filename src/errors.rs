use actix_web::{ResponseError, HttpResponse};
use derive_more::Display; 

#[derive(Display, Debug)]
pub enum ApiError {
    #[display("Paciente no encontrado.")]
    NotFound,

    #[display("Dato invalido {}.", _0)]
    BadRequest(String),

    #[display("Error interno del servidor.")]
    InternalServerError
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => {
                HttpResponse::NotFound().json(self.to_string())
            },
            ApiError::BadRequest(_) => {
                HttpResponse::BadRequest().json(self.to_string())
            },
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }  
}
