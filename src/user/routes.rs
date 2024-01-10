use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{
        get_user,
        create_user
    },
    AppState,
};

pub fn provide_user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/user", get(get_user))
        .route("/user-save", post(create_user))
        .with_state(app_state)
}