use std::fmt::{Display, Formatter};

use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use serde_derive::Serialize;

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub code: String,  
    pub cause: Option<Box<dyn std::error::Error>>,
    pub status: u16,
}
#[derive(Debug, Serialize)]
struct AppErrorBody {
    pub message: String,
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    pub code: String,
    pub cause: Option<String>, 
}

impl AppError {
    pub fn new(status: u16) -> Self {
        AppError {
            status,
            message: None,
            code: "UNKNOWN_ERROR".to_string(),
            cause: None,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn code(mut self, code: &str) -> Self {
        self.code = code.to_string();
        self
    }

    pub fn cause<E: std::error::Error + 'static>(mut self, cause: E) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.cause, &self.message) {
            (Some(cause), Some(message)) => write!(f, "{}: {}", message, cause),
            (Some(cause), None) => write!(f, "{}", cause),
            (None, Some(message)) => write!(f, "{}", message),
            (None, None) => write!(f, "{}", self.status_code().canonical_reason().unwrap()),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(AppErrorBody {
            message: self.message.clone().unwrap_or_else(|| "Something went wrong".to_string()),
            error: ErrorDetail {
                code: self.code.clone(), 
                cause: self.cause.as_ref().map(|e| e.to_string()), 
            },
        })
    }
}