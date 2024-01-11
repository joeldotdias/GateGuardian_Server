use std::sync::Arc;

use axum::{
    extract::{ Query, State },
    http::StatusCode,
    response::IntoResponse,
    Json
};
use serde_json::json;
use sqlx::Row;

use crate::{
    AppState,
    user::schema::{ CreateUserSchema, GetUserParams }
};

pub async fn get_user(
    State(data): State<Arc<AppState>>,
    Query(params): Query<GetUserParams>,
) -> impl IntoResponse {
    
    let query = format!("SELECT * FROM users WHERE email = '{}'",params.email);

    let user = match sqlx::query(&query)
        .fetch_one(&data.db)
        .await {   
            Ok(row) => row,
            Err(err) => {
                println!("Some db problem {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    "OOPSIE"
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

pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreateUserSchema>
) -> impl IntoResponse {
    
    let query_result = 
        sqlx::query(r#"INSERT INTO users (name, email, society, category) VALUES (?, ?, ?, ?)"#)
            .bind(payload.name.to_string())
            .bind(payload.email.to_string())
            .bind(payload.society.to_string())
            .bind(payload.category.to_string())
            .execute(&data.db)
            .await;

    if query_result.is_ok() {
        return (
            axum::http::StatusCode::CREATED,
            Json(json!({
                "message": "User created successfully"
            }))
        ).into_response();
    } else {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "err": "Could not create user"
            }))
        ).into_response();
    }
}