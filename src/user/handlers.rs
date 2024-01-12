use std::sync::Arc;

use axum::{
    extract::{ Query, State },
    response::IntoResponse,
    http,
    Json
};
use serde_json::json;

use crate::{
    AppState,
    user::{
        model::User,
        schema::{ CreateUserSchema, GetUserParams }
    }
};

pub async fn get_user(
    State(data): State<Arc<AppState>>,
    Query(params): Query<GetUserParams>,
) -> impl IntoResponse {
    
    let query = format!("SELECT * FROM users WHERE email = '{}'",params.email);

    // let user_response = json!({
    //     "id": user.try_get::<i64, _>("id").unwrap_or_default(),
    //     "name": user.try_get::<String, _>("name").unwrap_or_default(),
    //     "email": user.try_get::<String, _>("email").unwrap_or_default(),
    //     "category": user.try_get::<String, _>("category").unwrap_or_default(),
    //     "society": user.try_get::<String, _>("society").unwrap_or_default()
    // });
    let query_result = sqlx::query_as::<_, User>(&query)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            return (http::StatusCode::OK, Json(user)).into_response();
        }
        Err(err) => {
            dbg!("Error: {}", err);
            return (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch user"
                }))
            ).into_response();
        }
    };
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
            http::StatusCode::CREATED,
            Json(json!({
                "message": "User created successfully"
            }))
        ).into_response();
    } else {
        return (
            http::StatusCode::BAD_REQUEST,
            Json(json!({
                "err": "Could not create user"
            }))
        ).into_response();
    }
}