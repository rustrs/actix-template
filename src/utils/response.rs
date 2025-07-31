use serde::Serialize;
use actix_web::{HttpResponse, http::StatusCode};

use crate::utils::error_code::ErrorCode;




#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> HttpResponse {
        let res = ApiResponse {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        };
        HttpResponse::build(StatusCode::OK).json(res)
    }

    pub fn error(code: i32, message: &str) -> HttpResponse {
        let res = ApiResponse::<()> {
            code,
            message: message.to_string(),
            data: None,
        };
        HttpResponse::build(StatusCode::OK).json(res)
    }
    pub fn from_error(err: ErrorCode) -> HttpResponse {
        Self::error(err.code(), err.message())
    }
}