
use actix_web::{get, post, Responder,web,HttpRequest};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::app::state::AppState;

use crate::utils::{
    utils::verify_personal_sign,
    error_code::ErrorCode,
    response::ApiResponse,
};

use crate::models::user::{
    find_or_create_user
};

use crate::middleware::{
    auth::{
        generate_jwt,
        Claims
    },
};

use deadpool_redis::redis::AsyncCommands;
use crate::tpl::{
    message::MsgTemplate,
    redis_key::RedisKeyTemplate,
};


#[derive(Debug, Deserialize,Serialize)]
struct LoginRequest {
    address: String,
    sign: String,
    timestamp: u64,
}



#[post("/user/login")]
async fn user_login(
    mut req: web::Json<LoginRequest>,
    state: web::Data<AppState>) -> impl Responder {
    
    tracing::info!("req:{}",serde_json::json!(&req));

    req.address = req.address.to_lowercase();
    let message = MsgTemplate::Welcome(req.timestamp).format();
    let is_valid = match verify_personal_sign(&message, &req.sign,&req.address) {
        Ok(valid) => valid,
        Err(e) => {
            tracing::error!("verify_personal_sign:{}",e);
            return ApiResponse::<()>::from_error(ErrorCode::InvalidSignature);
        }
    };
    if !is_valid {
        return ApiResponse::<()>::from_error(ErrorCode::InvalidSignature);
    }
    let pool = &state.mysql;
    let user = match find_or_create_user(&pool, &req.address).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("find_or_create_user:{}",e);
            return ApiResponse::<()>::from_error(ErrorCode::UserNotFound)
        }
    };

    let jwt_config = state.config.jwt().clone();
    match generate_jwt(user.id, &user.address,jwt_config.expiration,jwt_config.secret) {
        Ok(token) => ApiResponse::success(serde_json::json!({
            "user": {
                "id": user.id,
                "address": user.address,
                "create_time": user.create_time,
            },
            "token": token,
            "token_type": "Bearer",
            "expire_time": jwt_config.expiration,
        })),
        Err(_) => ApiResponse::<()>::from_error(ErrorCode::ServerError),
    }
}


#[get("/user/profile")]
async fn user_profile(claims: web::ReqData<Claims>) -> impl Responder {
    ApiResponse::success(serde_json::json!({
        "user_id":claims.user_id,
        "address":claims.address,
        "exp":claims.exp,
    }))
}


#[post("/user/logout")]
async fn user_logout(req: HttpRequest, app_state: web::Data<AppState>,claims: web::ReqData<Claims>) -> impl Responder {
    let redis_pool = app_state.redis().clone();
    let token_opt = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.trim().to_string());

    let token = token_opt.unwrap();
    // 解码以获取过期时间
    let exp = claims.exp;
    let ttl = exp.saturating_sub(Utc::now().timestamp() as usize);

    // 将 token 加入 Redis 黑名单
    let mut conn = match redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get redis connection: {}", e);
            return ApiResponse::<()>::from_error(ErrorCode::ServerError);
        }
    };
    let redis_key = RedisKeyTemplate::Blacklist(token).format();
    if let Err(e) = conn.set_ex::<_, _, ()>(redis_key, "1", ttl as u64).await {
        tracing::error!("Failed to set redis blacklist: {}", e);
        return ApiResponse::<()>::from_error(ErrorCode::ServerError);
    }
    ApiResponse::success(serde_json::json!({}))
}