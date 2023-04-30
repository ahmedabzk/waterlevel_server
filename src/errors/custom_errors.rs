use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub enum CustomErrors {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError,
    UserDoesNotExist,
    UserAlreadyExist,
    Unauthorized,
    SqlxError(String),
}

impl From<sqlx::error::Error> for CustomErrors {
    fn from(value: sqlx::error::Error) -> Self {
        Self::SqlxError(value.to_string())
    }
}

impl IntoResponse for CustomErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "faild to create token"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credential"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
            Self::UserAlreadyExist => (StatusCode::BAD_REQUEST, "User already exist"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "authentication failed"),
            Self::SqlxError(_) => (StatusCode::BAD_REQUEST, "sqlx error"),
        };

        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
