use std::sync::Arc;

use axum::{
    extract::{ Query, State },
    response::IntoResponse,
    http::{self, HeaderMap},
    Json
};
use serde_json::json;
use sqlx::Row;

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
    
    let query = format!("
        SELECT u.user_id, u.name, u.email, u.category, soc.society_name AS society 
        FROM users AS u INNER JOIN societies AS soc ON u.society_id = soc.society_id 
        WHERE u.email = '{}'",params.email);

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
    headers: HeaderMap,
    Json(payload): Json<CreateUserSchema>,
) -> impl IntoResponse {

    let name = payload.name.as_str();
    let email = payload.email.as_str();
    let category = payload.category.as_str();
    let admin = headers.get("admin").unwrap();

    let society_id_query = format!("SELECT society_id FROM users WHERE email = {:?}", admin);
    
    let society_id = match sqlx::query(&society_id_query)
        .fetch_one(&data.db)
        .await {
            Ok(admin) => {
                admin.try_get::<i32, _>("society_id").unwrap()
            }
            Err(err) => {
                dbg!("Could not add resident: {}", err);
                    return (
                        http::StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "err": "This society has not yet been registered"
                        }))
                    ).into_response();
            }
    };
    
    let query_result = sqlx::query(r#"
        INSERT INTO users (name, email, society_id, category) 
        VALUES (?, ?, ?, ?)
    "#)
    .bind(name)
    .bind(email)
    .bind(society_id)
    .bind(category)
    .execute(&data.db)
    .await;
    
    match query_result {
        Err(err) => {
            dbg!("Could not add to the users table {}", err);
            return (
                http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Failed to register user"
                }))
            ).into_response();
        }
        _ => {}
    }
    
    match category {
        "resident" | "admin" => {
            let add_resident_query = sqlx::query(r#"INSERT INTO residents ( email) VALUES (?)"#)
                .bind(email)
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
                    dbg!("Could not add resident: {}", err);
                    return (
                        http::StatusCode::BAD_REQUEST,
                        Json(json!({
                            "err": "Failed to register resident"
                        }))
                    ).into_response();
                }
            }
        }

        "security" => {
            let add_security_query = sqlx::query(r#"INSERT INTO securities (email) VALUES (?)"#)
                .bind(email)
                .execute(&data.db)
                .await;

            match add_security_query {
                Ok(_) => {
                    return (
                        http::StatusCode::OK,
                        Json(json!({
                            "msg": "Security registered successfully"
                        }))
                    ).into_response();
                }
                Err(err) => {
                    dbg!("Could not add security: {}", err);
                    return (
                        http::StatusCode::BAD_REQUEST,
                        Json(json!({
                            "err": "Failed to register security"
                        }))
                    ).into_response();
                }
            }
        }
        _ => {
            return (
                http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "The specified category did not match any of the available options"
                }))
            ).into_response();
        }
    }
}
