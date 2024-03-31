use std::sync::Arc;

use axum::{
    routing::{ get, post },
    middleware,
    Router,
};

use crate::{
    config::AppState,
    middleware::sanitize_headers,
    user::handlers::{ get_user, create_user }
};


pub fn provide_user_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/user-save", post(create_user))
        .route_layer(middleware::from_fn(sanitize_headers))        
        .route("/user", get(get_user))
        .with_state(Arc::clone(app_state))
}
