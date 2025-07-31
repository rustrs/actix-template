pub mod user;
pub mod health;

use actix_web::web;
use crate::middleware::auth;


pub fn init(cfg: &mut web::ServiceConfig,jwt:auth::JwtMiddleware) {
    cfg
        .service(user::routes(jwt.clone()))
        .service(health::routes(jwt.clone()));
}