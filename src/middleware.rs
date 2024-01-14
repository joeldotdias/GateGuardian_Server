use axum::http::Method;
use tower_http::{
    trace::{self, TraceLayer},
    cors::{ Any, CorsLayer },
    classify::{SharedClassifier, ServerErrorsAsFailures}
};
use tracing::Level;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
}

pub fn logger()  -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>>{
    println!("Logs");
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new()
            .level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new()
            .level(Level::INFO))
}