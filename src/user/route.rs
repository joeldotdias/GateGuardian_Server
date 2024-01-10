use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{
        health_checker,
        get_user
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthchecker", get(health_checker))
        .route("/user", get(get_user))
        .with_state(app_state)
}
