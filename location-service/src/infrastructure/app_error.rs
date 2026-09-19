use thiserror::Error;
use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode};
use std::env::VarError;
use sqlx::Error as SqlxError;
use crate::domain::location_errors::DomainError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("Environment variable error: {0}")]
    Var(#[from] VarError),
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    #[error("Parse error on environment variable: {0}")]
    Parse(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "testing 123".to_string()).into_response()
    }
}
