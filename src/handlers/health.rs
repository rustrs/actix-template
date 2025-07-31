use actix_web::{get, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    tracing::info!("Health check endpoint hit");
    "OK"
}