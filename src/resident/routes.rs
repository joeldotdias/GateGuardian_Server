use std::sync::Arc;

use axum::{
    middleware, routing::{ get, post, put }, Router
};

use crate::{
    config::AppState,
    middleware::sanitize_headers,
    resident::handlers::{
        add_notice, add_resident_home_details, get_dashboard_details, get_notices,
        get_recent_regular_otp, get_recent_visitor_otp, get_regulars, get_resident_by_email,
        get_residents_by_society, get_security_by_society, get_visitors, save_regular,
        save_visitor, update_resident_pfp, update_resident_profile
    }
};


pub fn provide_resident_routes(app_state: &Arc<AppState>) -> Router {

    Router::new()
        .route("/sign-in", get(get_resident_by_email))
        .route("/dashboard", get(get_dashboard_details))
        .route("/update-home", put(add_resident_home_details))
        .route("/update-profile", put(update_resident_profile))
        .route("/update-pfp", put(update_resident_pfp))
        .route("/visitors", get(get_visitors))
        .route("/visitor-save", post(save_visitor))
        .route("/visitor-recent", get(get_recent_visitor_otp))
        .route("/regulars", get(get_regulars))
        .route("/regular-save", post(save_regular))
        .route("/regular-recent", get(get_recent_regular_otp))
        .route("/admin/notice-save", post(add_notice))
        .route("/notices", get(get_notices))
        .route("/admin/residents", get(get_residents_by_society))
        .route("/admin/securities", get(get_security_by_society))
        .route_layer(middleware::from_fn(sanitize_headers))
        .with_state(Arc::clone(app_state))
}
