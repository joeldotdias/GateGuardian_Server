use std::sync::Arc;

use axum::{
    routing::{ get, post, put },
    Router
};

use crate::{
    AppState,
    security::handlers::{
        get_security_by_email,
        get_visitors, verified_visitor_to_logs, get_visitor_logs,
        update_security_profile, update_security_pfp
    }
};


pub fn provide_security_routes(app_state: &Arc<AppState>) -> Router {

    Router::new()
        .route("/sign-in", get(get_security_by_email))
        .route("/visitors", get(get_visitors))
        .route("/visitor-verified", post(verified_visitor_to_logs))
        .route("/visitor-logs", get(get_visitor_logs))
        .route("/update-profile", put(update_security_profile))
        .route("/update-pfp", put(update_security_pfp))
        .with_state(app_state.clone())
}