use axum::{
    extract::Request,
    http::Method,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_http::{
    trace::{ self, TraceLayer },
    cors::{ Any, CorsLayer },
    classify::{ SharedClassifier, ServerErrorsAsFailures }
};
use tracing::Level;

use crate::error::GGError;

#[derive(Clone)]
pub struct CurrUser {
    pub email: String
}

impl From<String> for CurrUser {
    fn from(email: String) -> Self {
        CurrUser { email }
    }
}

pub async fn sanitize_headers(mut req: Request, next: Next) -> Result<Response, impl IntoResponse> {
    let header_val = req.headers()
        .get("email")
        .and_then(|header| header.to_str().ok());

    let header_str = if let Some(header_val) = header_val {
        header_val.to_owned()
    } else {
        return Err(
            GGError::NecessaryHeadersAbsent.into_response()
        );
    };
    
    req.extensions_mut().insert(CurrUser::from(header_str));
    Ok(next.run(req).await)
}

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
}

pub fn logger() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>>{
    println!("HERE BE THE LOGS");
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new()
            .level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new()
            .level(Level::INFO))
}
