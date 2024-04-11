use axum::{
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json
};
use serde_json::{ json, Value };

// #[derive(Clone)]
// pub enum GGError {
//     NecessaryHeadersAbsent,
//     DefunctCredentials(String),
//     ServerError(String),
//     RegistrationFailure(String),
//     Stupidity(String)
// }

pub enum GGError<'a> {
    NecessaryHeadersAbsent,
    DefunctCredentials(&'a str),
    ServerError(&'a str),
    RegistrationFailure(&'a str),
    Stupidity(&'a str)
}

impl IntoResponse for GGError<'_> {
    fn into_response(self) -> Response {
        let (status, message) = (self.status(), self.err_msg());
        (status, message).into_response()
    }
}

impl GGError<'_> {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NecessaryHeadersAbsent => StatusCode::PRECONDITION_FAILED,
            Self::ServerError(_)
            | Self::RegistrationFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
