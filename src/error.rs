use axum::{
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json
};
use serde_json::{ json, Value };

#[derive(Clone)]
pub enum GGError {
    NecessaryHeadersAbsent,
    DefunctCredentials(String),
    ServerError(String),
    RegistrationFailure(String),
    Stupidity(String)
}

impl IntoResponse for GGError {
    fn into_response(self) -> Response {
        let (status, message) = (self.status(), self.err_msg());
        (status, message).into_response()
    }
}

impl GGError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NecessaryHeadersAbsent => StatusCode::PRECONDITION_FAILED,
            Self::ServerError(_) | Self::RegistrationFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DefunctCredentials(_) => StatusCode::UNAUTHORIZED,
            Self::Stupidity(_) => StatusCode::BAD_REQUEST
        }
    }
    
    fn err_msg(&self) -> axum::Json<Value>{
        let msg = match self {
            Self::NecessaryHeadersAbsent => "Could not detect required headers",
            Self::ServerError(msg)
            | Self::DefunctCredentials(msg)
            | Self::RegistrationFailure(msg)
            | Self::Stupidity(msg) => msg
        };
        
        Json(json!({
            "err": msg
        }))
    }
}
