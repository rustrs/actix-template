pub const CODE_INVALID_SIGNATURE: i32 = 4001;
pub const CODE_USER_NOT_FOUND: i32 = 4002;
pub const CODE_SERVER_ERROR: i32 = 4003;
pub const CODE_UNAUTHORIZED: i32 = 4004;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ErrorCode {
    InvalidSignature,
    UserNotFound,
    ServerError,
    Unauthorized
}
#[deny(unreachable_patterns)]
impl ErrorCode {
    pub fn code(&self) -> i32 {
        match self {
            ErrorCode::InvalidSignature => CODE_INVALID_SIGNATURE,
            ErrorCode::UserNotFound => CODE_USER_NOT_FOUND,
            ErrorCode::ServerError => CODE_SERVER_ERROR,
            ErrorCode::Unauthorized =>CODE_UNAUTHORIZED,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::InvalidSignature => "Invalid signature",
            ErrorCode::UserNotFound => "User not found",
            ErrorCode::ServerError => "Server error",
            ErrorCode::Unauthorized =>"Unauthorized",
        }
    }
}