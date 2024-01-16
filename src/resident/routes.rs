use std::sync::Arc;

use axum::{
    routing::{ get, post, put },
    Router
};

use crate::{
    AppState,
    resident::handlers::{
        get_resident_by_email, 
        add_resident_home_details, update_resident_profile,  update_resident_pfp,
        get_visitors, save_visitor, get_recent_visitor_otp,
        get_resident_by_society
    }
};


pub fn provide_resident_routes(app_state: &Arc<AppState>) -> Router {

    Router::new()
        .route("/sign-in", get(get_resident_by_email))//.route_layer(middleware::from_fn_with_state(app_state.clone(), jwt_auth)))
        .route("/update-home", put(add_resident_home_details))
        .route("/update-profile", put(update_resident_profile))
        .route("/update-pfp", put(update_resident_pfp))
        .route("/visitors", get(get_visitors))
        .route("/visitor-save", post(save_visitor))
        .route("/visitor-recent", get(get_recent_visitor_otp))
        .route("/admin/residents", get(get_resident_by_society))
        .with_state(app_state.clone())
}