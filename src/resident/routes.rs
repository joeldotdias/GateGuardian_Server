use std::sync::Arc;

use axum::{
    routing::{ get, post, put },
    Router
};

use crate::{
    AppState,
    resident::handlers::{ get_resident_by_email, update_resident_pfp, get_visitors }
};

pub fn provide_resident_routes(app_state: &Arc<AppState>) -> Router {

    Router::new()
        .route("/sign-in", get(get_resident_by_email))
        .route("/update-pfp", put(update_resident_pfp))
        .route("/visitors", get(get_visitors))
        .with_state(app_state.clone())
}