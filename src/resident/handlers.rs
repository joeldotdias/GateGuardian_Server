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
    resident::{
        model::Resident,
        schema::UpdatePfpParams
    },
};

// App entry
pub async fn get_resident_by_email(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let query = format!("SELECT * FROM residents WHERE email = {:?}", headers.get("email").unwrap());

    let query_result = sqlx::query_as::<_, Resident>(&query)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(resident) => {
            return (axum::http::StatusCode::OK, Json(resident)).into_response();
        }
        Err(err) => {
            dbg!("Error: {}", err);
            return(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch resident data"
                }))
            ).into_response();
        }     
    };
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
        ", params.pfpUrl.to_string(), headers.get("email").unwrap());
    
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
                    axum::http::StatusCode::BAD_REQUEST,
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
                "otp": row.try_get::<String, _>("otp").unwrap_or_default()
            })
        }).collect();
    
    (axum::http::StatusCode::OK, Json(visitors)).into_response()
}
