use std::sync::Arc;

use axum::Router;

use crate::{
    AppState,
    middleware::{ self },
    user, resident
};

pub async fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user::routes::provide_user_routes(&app_state))
        .nest("/resident", resident::routes::provide_resident_routes(&app_state))
        .layer(middleware::cors_layer())
        .layer(middleware::logger())
}