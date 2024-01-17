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

    let name = payload.name.as_str();
    let email = payload.email.as_str();
    let society = payload.society.as_str();
    let category = payload.category.as_str();

    let query_result = 
        sqlx::query(r#"INSERT INTO users (name, email, society, category) VALUES (?, ?, ?, ?)"#)
            .bind(name)
            .bind(email)
            .bind(society)
            .bind(category)
            .execute(&data.db)
            .await;
    
    if query_result.is_err() {
        return (
            http::StatusCode::BAD_REQUEST,
            Json(json!({
                "err": "Failed to register user"
            }))
        ).into_response();
    }

    match category {
        "resident" | "admin" => {
            let add_resident_query = sqlx::query(r#"INSERT INTO residents (name, email, society) VALUES (?, ?, ?)"#)
                .bind(name)
                .bind(email)
                .bind(society)
                .execute(&data.db)
                .await;

            match add_resident_query {
                Ok(_) => {
                    return (
                        http::StatusCode::OK,
                        Json(json!({
                            "msg": "Resident registered successfully"
                        }))
                    ).into_response();
                }
                Err(err) => {
                    dbg!("Could not add resident{}", err);
                    return (
                        http::StatusCode::BAD_REQUEST,
                        Json(json!({
                            "err": "Failed not create user"
                        }))
                    ).into_response();
                }
            }
        }
        "security" => {
            let add_security_query = sqlx::query(r#"INSERT INTO securities (name, email, society) VALUES (?, ?, ?)"#)
                .bind(name)
                .bind(email)
                .bind(society)
                .execute(&data.db)
                .await;

            match add_security_query {
                Ok(_) => {
                    return (
                        http::StatusCode::OK,
                        Json(json!({
                            "msg": "Resident registered successfully"
                        }))
                    ).into_response();
                }
                Err(err) => {
                    dbg!("Could not add resident{}", err);
                    return (
                        http::StatusCode::BAD_REQUEST,
                        Json(json!({
                            "err": "Failed not create user"
                        }))
                    ).into_response();
                }
            }
        }
        _ => {
            return (
                http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Category field did not match any of the available options"
                }))
            ).into_response();
        }
    }
}