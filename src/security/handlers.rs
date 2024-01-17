use std::sync::Arc;

use axum::{
    extract::{ Query, State },
    http::header::HeaderMap,
    response::IntoResponse,
    Json
};

use serde_json::json;
use sqlx::Row;

use crate::{
    AppState,
    security::{
        model::Security,
        schema::{ UpdateSecurityProfileSchema, UpdatePfpParams }
    }
};


// App entry
pub async fn get_security_by_email(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    
    let query = format!("SELECT * FROM securities WHERE email = {:?}", headers.get("email").unwrap());

    let query_result = sqlx::query_as::<_, Security>(&query)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(security) => {
            return (axum::http::StatusCode::OK, Json(security)).into_response();
        }
        Err(err) => {
            dbg!("Error: {} {}", err, query);
            return(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch security data"
                }))
            ).into_response();
        }     
    };
}


// Profile
pub async fn update_security_profile(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateSecurityProfileSchema>
) -> impl IntoResponse {
    let query = format!("
            UPDATE securities
            SET name = '{}', badge_id = '{}', phone_no = '{}'
            WHERE email = {:?}
        ", payload.name, payload.about_me, payload.phone_no, headers.get("email").unwrap());

    let query_result = sqlx::query(&query)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            (
                axum::http::StatusCode::OK,
                Json(json!({
                    "message": "Profile updated successfully"
                }))
            ).into_response()
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update pfp"
                }))
            ).into_response()
        }
    }
}

pub async fn update_security_pfp(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<UpdatePfpParams>
) -> impl IntoResponse {

    let query = format!("
            UPDATE securities
            SET pfp_url = '{}'
            WHERE email = {:?}
        ", params.pfpUrl.to_string(), headers.get("email").unwrap());
    
    let query_result = sqlx::query(&query)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            return (
                axum::http::StatusCode::OK,
                Json(json!({
                    "message": "Pfp updated successfully"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("err: {}\nquery:{}", err, &query);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update pfp"
                }))
            ).into_response();
        }
    }
}