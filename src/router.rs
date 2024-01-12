use std::sync::Arc;

use axum::Router;

use crate::{
    AppState,
    user, resident
};

pub async fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user::routes::provide_user_routes(&app_state))
        .merge(resident::routes::provide_resident_routes(&app_state))
}