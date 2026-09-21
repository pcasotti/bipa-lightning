use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error")]
    Database(#[from] crate::db::error::Error),
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{self}");
        let status = match &self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorBody {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
