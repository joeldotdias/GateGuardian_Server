use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    config::AppState,
    middleware::sanitize_headers,
    user::handlers::{create_user, get_user},
};

pub fn provide_user_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/user-save", post(create_user))
        .route_layer(middleware::from_fn(sanitize_headers))
        .route("/user", get(get_user))
        .with_state(Arc::clone(app_state))
}
