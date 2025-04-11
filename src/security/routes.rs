use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::{
    config::AppState,
    middleware::sanitize_headers,
    security::handlers::{
        get_regulars, get_residents_to_notify, get_security_by_email, get_visitor_logs,
        get_visitors, update_security_pfp, update_security_profile, verified_visitor_to_logs,
    },
};

pub fn provide_security_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/sign-in", get(get_security_by_email))
        .route("/visitors", get(get_visitors))
        .route("/visitor-logs", get(get_visitor_logs))
        .route("/regulars", get(get_regulars))
        .route("/notify", get(get_residents_to_notify))
        .route("/update-profile", put(update_security_profile))
        .route("/update-pfp", put(update_security_pfp))
        .route_layer(middleware::from_fn(sanitize_headers))
        .route("/visitor-verified", post(verified_visitor_to_logs))
        .with_state(Arc::clone(app_state))
}

