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
        model::{ Security, VisitorLog },
        schema::{
            UpdateSecurityProfileSchema, UpdatePfpParams,
            VisitorSecurityDto, VerifiedVisitorParams, VerifiedVisitorDetails
        }
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


// Visitors
pub async fn get_visitors(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    let security_society_query = format!("SELECT society from securities WHERE email = {:?}", headers.get("email").unwrap());

    let security_society = match sqlx::query(&security_society_query)
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<String, _>("society").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{}", err);
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let query = format!("
            SELECT visitor_id, name, host_flat, host_building, host_society, otp
            FROM visitors
            WHERE host_society = {:?}
        ", security_society);

    let query_result = sqlx::query_as::<_, VisitorSecurityDto>(&query)
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(visitors) => {
            return (
                axum::http::StatusCode::OK,
                Json(visitors)
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch visitors data: {}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch visitors data"
                }))
            ).into_response();
        }
    }
}

pub async fn verified_visitor_to_logs(
    State(data): State<Arc<AppState>>,
    Query(params): Query<VerifiedVisitorParams>
) -> impl IntoResponse {
    let visitor_id = params.visitor_id.parse::<i32>().unwrap();

    let get_visitor_data_query = format!("SELECT name, phone_no, host_flat, host_building, host_society FROM visitors WHERE visitor_id = {:?}", visitor_id);
    let visitor_data = match sqlx::query_as::<_, VerifiedVisitorDetails>(&get_visitor_data_query)
        .fetch_one(&data.db)
        .await {
            Ok(visitor_data) => visitor_data,
            Err(err) => {
                dbg!("Could not fetch visitor data:{}", err);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "err": "Could not fetch visitor data with provided id"
                    }))
                ).into_response();
            }
    };
    
    let remove_from_visitors_query = format!("DELETE FROM visitors WHERE visitor_id = {:?}", visitor_id);
    
    let remove_visitor_result = sqlx::query(&remove_from_visitors_query)
        .execute(&data.db)
        .await;

    match remove_visitor_result {
        Err(err) => {
            dbg!("Could not remove visitor :{}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not move the visitor to logs"
                }))
            ).into_response();
        }
        _ => {}
    }

    let entry = chrono::offset::Utc::now();

    let add_visitor_to_logs_result = sqlx::query(r#"
        INSERT INTO visitor_logs (name, phone_no, host_flat, host_building, host_society, entry)
        VALUES (?, ?, ?, ?, ?, ?)"#
    ).bind(visitor_data.name)
    .bind(visitor_data.phone_no)
    .bind(visitor_data.host_flat)
    .bind(visitor_data.host_building)
    .bind(visitor_data.host_society)
    .bind(entry)
    .execute(&data.db)
    .await;
    
    match add_visitor_to_logs_result {
        Ok(_) => {
            return (
                axum::http::StatusCode::OK,
                Json(json!({
                    "msg": "Successfully moved visitor to logs"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch visitors data: {}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch visitors data"
                }))
            ).into_response();
        }
    }
}

pub async fn get_visitor_logs(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    let security_society_query = format!("SELECT society from securities WHERE email = {:?}", headers.get("email").unwrap());

    let security_society = match sqlx::query(&security_society_query)
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<String, _>("society").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{}", err);
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let query = format!("SELECT * FROM visitor_logs WHERE host_society = {:?}", security_society);

    let query_result = sqlx::query_as::<_, VisitorLog>(&query)
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(visitor_logs) => {
            return (
                axum::http::StatusCode::OK,
                Json(visitor_logs)
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch visitors data: {}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch visitors data"
                }))
            ).into_response();
        }
    }
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
            return (
                axum::http::StatusCode::OK,
                Json(json!({
                    "message": "Profile updated successfully"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update pfp"
                }))
            ).into_response();
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