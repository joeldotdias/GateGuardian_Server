use std::sync::Arc;

use axum::{
    routing::{ get, post, put },
    Router
};

use crate::{
    config::AppState,
    resident::handlers::{
        get_resident_by_email, get_dashboard_details,
        add_resident_home_details, update_resident_profile,  update_resident_pfp,
        add_notice, get_notices,
        get_visitors, save_visitor, get_recent_visitor_otp,
        get_regulars, save_regular, get_recent_regular_otp,
        get_residents_by_society, get_security_by_society
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
        .with_state(app_state.clone())
}
