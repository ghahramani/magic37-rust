use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Not found")]
    NotFound,
    #[display("Internal error: {}", _0)]
    Internal(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::NotFound => HttpResponse::NotFound().finish(),
            ServiceError::Internal(msg) => {
                HttpResponse::InternalServerError().body(msg.to_string())
            }
        }
    }
}

pub trait OptionExt<T> {
    fn ok_or_not_found(self) -> Result<T, ServiceError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self) -> Result<T, ServiceError> {
        self.ok_or(ServiceError::NotFound)
    }
}
