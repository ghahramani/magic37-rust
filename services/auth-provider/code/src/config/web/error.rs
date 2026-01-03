use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use http_api_problem::{HttpApiProblem, StatusCode as ProblemStatus};

#[derive(Debug, Display)]
pub enum HttpError {
    #[display("Not found")]
    NotFound,
    #[display("Internal error: {}", _0)]
    Internal(String),
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::NotFound => StatusCode::NOT_FOUND,
            HttpError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let problem = match self {
            HttpError::NotFound => HttpApiProblem::new(ProblemStatus::NOT_FOUND)
                .title("Resource not found")
                .detail("The requested resource does not exist")
                .type_url("/probs/not-found"),

            HttpError::Internal(msg) => HttpApiProblem::new(ProblemStatus::INTERNAL_SERVER_ERROR)
                .title("Internal server error")
                .detail(msg)
                .type_url("/probs/internal-error"),
        };

        problem.to_actix_response()
    }
}

/// Helper trait to convert Option → Result<ServiceError>
pub trait OptionExt<T> {
    fn ok_or_not_found(self) -> Result<T, HttpError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self) -> Result<T, HttpError> {
        self.ok_or(HttpError::NotFound)
    }
}
