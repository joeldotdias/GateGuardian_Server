use std::sync::Arc;

use axum::{
    extract::{ Query, State },
    http::{ header::HeaderMap, StatusCode },
    response::IntoResponse,
    Json
};

use serde_json::json;
use sqlx::Row;

use crate::{
    AppState,
    resident::schema::UpdatePfpParams,
};

// App entry
pub async fn get_resident_by_email(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let query = format!("SELECT * FROM residents WHERE email = {:?}", headers.get("email").unwrap());

    let resident = match sqlx::query(&query)
        .fetch_one(&data.db)
        .await {
            Ok(row) => row,
            Err(err) => {
                dbg!("Couldn't read data {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "WHOOPS"
                )
                .into_response();
            }
    };

    let resident_response = json!({
        "residentId": resident.try_get::<i64, _>("resident_id").unwrap_or_default(),
        "name": resident.try_get::<String, _>("name").unwrap_or_default(),
        "email": resident.try_get::<String, _>("email").unwrap_or_default(),
        "pfpUrl": resident.try_get::<String, _>("pfp_url").unwrap_or_default(),
        "aboutMe": resident.try_get::<String, _>("about_me").unwrap_or_default(),
        "phoneNo": resident.try_get::<String, _>("phone_no").unwrap_or_default(),
        "flatNo": resident.try_get::<i32, _>("flat_no").unwrap_or_default(),
        "building": resident.try_get::<String, _>("building").unwrap_or_default(),
        "society": resident.try_get::<String, _>("society").unwrap_or_default()
    });      
    
    (axum::http::StatusCode::OK, Json(resident_response)).into_response()
}


// Profile
pub async fn update_resident_pfp(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<UpdatePfpParams>
) -> impl IntoResponse {

    let query = format!("
            UPDATE residents
            SET pfp_url = '{}' 
            WHERE email = {:?}
        ", params.pfp_url.to_string(), headers.get("email").unwrap());
    
    let query_result = sqlx::query(&query)
        .execute(&data.db)
        .await;

    if query_result.is_err() {
        dbg!(&query);
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "err": "Could not update pfp"
            }))
        ).into_response();
    } else { 
        return (
            axum::http::StatusCode::OK,
            Json(json!({
                "message": "Pfp updated successfully"
            }))
        ).into_response();
    }
}


// Visitors
pub async fn get_visitors(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    let query = format!("SELECT * FROM visitors WHERE host_email = {:?}", headers.get("email").unwrap());

    let query_result = match sqlx::query(&query)
        .fetch_all(&data.db)
        .await {
            Ok(rows) => rows,
            Err(err) => {
                dbg!("Couldn't read data {}", err);
                return (
                    StatusCode::BAD_REQUEST,
                    "WHOOPS"
                ).into_response();
            }
        };

    let visitors: Vec<serde_json::Value> = query_result
        .into_iter()
        .map(|row| {
            json!({
                "visitorId": row.try_get::<i64, _>("visitor_id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "phoneNo": row.try_get::<String, _>("phone_no").unwrap_or_default(),
                "hostEmail": row.try_get::<String, _>("host_email").unwrap_or_default(),
                "hostFlat": row.try_get::<i32, _>("host_flat").unwrap_or_default(),
                "hostBuilding": row.try_get::<String, _>("host_building").unwrap_or_default(),
                "society": row.try_get::<String, _>("host_society").unwrap_or_default(),
                "otp": row.try_get::<String, _>("otp").unwrap_or_default()
            })
        }).collect();
    
    (axum::http::StatusCode::OK, Json(visitors)).into_response()
}