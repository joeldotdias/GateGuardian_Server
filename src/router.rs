use std::sync::Arc;

use axum::Router;

use crate::{
    config::AppState,
    middleware,
    user, resident, security
};

pub async fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user::routes::provide_user_routes(&app_state))
        .nest("/resident", resident::routes::provide_resident_routes(&app_state))
        .nest("/security", security::routes::provide_security_routes(&app_state))
        .layer(middleware::cors_layer())
        .layer(middleware::logger())
}
