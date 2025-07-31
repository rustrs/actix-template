use actix_web::{web};

use crate::handlers::user::{user_login, user_logout, user_profile};
use crate::middleware::auth;


pub fn routes(jwt: auth::JwtMiddleware) -> actix_web::Scope {
    web::scope("/api")
        .service(user_login)
        .service(
            web::scope("").
            wrap(jwt).
            service(user_profile).
            service(user_logout),
        )
}


