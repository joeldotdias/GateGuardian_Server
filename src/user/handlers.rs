use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use serde_json::json;
use sqlx::Row;

use crate::{
    schema::CreateUserSchema,
    AppState,
};

use super::schema::ParamOptions;
pub async fn health_checker() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "foo",
        "message": "bar"
    });
    Json(json_response)
}

pub async fn get_user(
    State(data): State<Arc<AppState>>,
    Query(params): Query<ParamOptions>,
) -> impl IntoResponse {
    let query = format!("SELECT * FROM users WHERE email = '{}'",params.email);

    let user = match sqlx::query(&query)
        .fetch_one(&data.db)
        .await {   
            Ok(row) => row,
            Err(err) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    "OOPs"
                )
                .into_response()
            }
    };

    let user_response = json!({
        "id": user.try_get::<i64, _>("id").unwrap_or_default(),
        "name": user.try_get::<String, _>("name").unwrap_or_default(),
        "email": user.try_get::<String, _>("email").unwrap_or_default(),
        "category": user.try_get::<String, _>("category").unwrap_or_default(),
        "society": user.try_get::<String, _>("society").unwrap_or_default()
    });

    (axum::http::StatusCode::OK, Json(user_response)).into_response()
}