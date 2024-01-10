use std::sync::Arc;

use axum::Router;

use crate::{user, AppState};

pub async fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user::routes::provide_user_routes(app_state))
}