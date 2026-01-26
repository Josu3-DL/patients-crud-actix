mod models;
mod handlers;
mod routes;
mod state;

use actix_web::{App, HttpServer, web};
use state::AppState;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        pacientes: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::pacientes_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
