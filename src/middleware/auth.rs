

use jsonwebtoken::{encode, EncodingKey, Header};


use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform, forward_ready},
    Error, HttpMessage,
};
use deadpool_redis::Pool;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::rc::Rc;
use redis::AsyncCommands;
use serde::{Deserialize,Serialize};

use crate::tpl::redis_key::RedisKeyTemplate;


#[derive(Serialize, Deserialize,Clone)]
pub struct Claims {
    pub user_id: u64,
    pub address: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id:u64,address: &str,expiration:u64,secret:String) 
-> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        user_id: user_id,
        address: address.to_string(),
        exp: chrono::Utc::now().timestamp() as usize + expiration as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}






#[derive(Clone)]
pub struct JwtMiddleware {
    pub secret: String,
    pub redis_pool:Pool,
}


impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
            secret: self.secret.clone(),
            redis_pool:self.redis_pool.clone(),
        })
    }
}




pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
    secret: String,
    redis_pool:Pool,
}

impl<S, B> actix_service::Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let secret = self.secret.clone();
        let redis_pool = self.redis_pool.clone();

        Box::pin(async move {
            // 提取 Authorization 头
            let token_opt = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .map(|s| s.trim().to_string());

            let token = match token_opt {
                Some(t) => t,
                None => return Err(actix_web::error::ErrorUnauthorized("Missing token")),
            };

            let result = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            );

            match result {
                Ok(token_data) => {
                    // 插入到 request.extensions
                    let mut conn = redis_pool.get().await.map_err(|_| {
                        actix_web::error::ErrorInternalServerError("Token is blacklisted")
                    })?;
                    
                    let redis_key = RedisKeyTemplate::Blacklist(token).format();
                    let exists: bool = conn.exists(redis_key).await.map_err(|_| {
                        actix_web::error::ErrorInternalServerError("Token is blacklisted")
                    })?;

                    if exists {
                        return Err(actix_web::error::ErrorUnauthorized("Token is blacklisted"));
                    }

                    req.extensions_mut().insert(token_data.claims);
                    service.call(req).await
                }
                Err(err) => {
                    match *err.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                            Err(actix_web::error::ErrorUnauthorized("Token expired"))
                        }
                        jsonwebtoken::errors::ErrorKind::InvalidToken => {
                            Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                        }
                        _ => Err(actix_web::error::ErrorUnauthorized("Token validation error")),
                    }
                }
            }
            
        })
    }
}


