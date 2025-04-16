use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request. {0}")]
    BadRequest(&'static str),

    #[error("EVM call failed. {0}")]
    EvmCallFailed(String),

    // Consider using Internal server error from underlying error instead of str.
    // #[error("Internal server error.")]
    // Internal(#[from] anyhow::Error),
    #[error("Internal server error. {0}")]
    Internal(String),

    #[error("Invalid address format. {0}")]
    InvalidAddressFormat(String),

    #[error("Signer missing or invalid. {0}")]
    SignerExtractionError(&'static str),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match &self {
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::EvmCallFailed(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::InvalidAddressFormat(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::SignerExtractionError(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
        };

        let body = Json(json!({ "error": error_msg }));
        (status, body).into_response()
    }
}
