use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
    
    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Image processing error: {0}")]
    ImageError(String),
    
    #[error("OCR processing error: {0}")]
    OcrError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError(_) => HttpResponse::InternalServerError().json(ErrorResponse::new(self)),
            AppError::EnvError(_) => HttpResponse::InternalServerError().json(ErrorResponse::new(self)),
            AppError::IoError(_) => HttpResponse::InternalServerError().json(ErrorResponse::new(self)),
            AppError::JsonError(_) => HttpResponse::BadRequest().json(ErrorResponse::new(self)),
            AppError::ImageError(_) => HttpResponse::BadRequest().json(ErrorResponse::new(self)),
            AppError::OcrError(_) => HttpResponse::BadRequest().json(ErrorResponse::new(self)),
            AppError::AuthError(_) => HttpResponse::Unauthorized().json(ErrorResponse::new(self)),
            AppError::NotFound(_) => HttpResponse::NotFound().json(ErrorResponse::new(self)),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(ErrorResponse::new(self)),
            AppError::InternalServerError(_) => HttpResponse::InternalServerError().json(ErrorResponse::new(self)),
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    fn new(error: &AppError) -> Self {
        ErrorResponse {
            error: error.to_string(),
        }
    }
} 