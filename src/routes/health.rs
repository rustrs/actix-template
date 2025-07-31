use actix_web::{ web};

use crate::handlers::health::health_check;

use crate::middleware::auth;

pub fn routes(_jwt: auth::JwtMiddleware) -> actix_web::Scope {
    web::scope("")
        .service(health_check)
}