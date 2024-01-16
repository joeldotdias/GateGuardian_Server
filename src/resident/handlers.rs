use std::sync::Arc;
use rand::Rng;

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
        schema::{ AddHomeDetailsSchema, UpdatePfpParams, VisitorResidentDto, SaveVisitorSchema, ResidentDetailsSchema }
    },
};

use super::schema::UpdateResidentProfileSchema;


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
pub async fn add_resident_home_details(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<AddHomeDetailsSchema>
) -> impl IntoResponse {

    let query = format!("
            UPDATE residents
            SET flat_no = {}, building = '{}'
            WHERE email = {:?}
        ", payload.flat, payload.building, headers.get("email").unwrap());

    let query_result = sqlx::query(&query)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            (
                axum::http::StatusCode::OK,
                Json(json!({
                    "message": "Home details updated successfully"
                }))
            ).into_response()
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update home details"
                }))
            ).into_response()
        }
    }
}

pub async fn update_resident_profile(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateResidentProfileSchema>
) -> impl IntoResponse {
    let query = format!("
            UPDATE residents
            SET name = '{}', about_me = '{}', phone_no = '{}'
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

    let query_result = sqlx::query_as::<_, VisitorResidentDto>(&query)
        .fetch_all(&data.db)
        .await;
    
    match query_result {
        Ok(rows) => {
            return(
                axum::http::StatusCode::OK,
                Json(rows)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't read data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                "WHOOPS"
            ).into_response();
        }
    };
}

pub async fn save_visitor(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<SaveVisitorSchema>
) -> impl IntoResponse {
    
    let resident_details: ResidentDetailsSchema;
    let resident_details_query = format!("SELECT flat_no, building, society FROM residents WHERE email = {:?}", payload.host_email.to_string());
    
    let resident_details_query_result = sqlx::query_as::<_, ResidentDetailsSchema>(&resident_details_query)
        .fetch_one(&data.db)
        .await;

    
    
    match resident_details_query_result {
        Ok(details) => {
            resident_details = details;
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                "WHOOPS"
            ).into_response();
        }
    }

    let otp = rand::thread_rng().gen_range(100000..=999999);

    let save_visitor_query_result = sqlx::query(r#"
        INSERT INTO visitors (name, phone_no, host_email, host_flat, host_building, host_society, otp)
        VALUES (?, ?, ?, ?, ?, ?, ?)"#
    ).bind(payload.name.to_string())
    .bind(payload.phone_no.to_string())
    .bind(payload.host_email.to_string())
    .bind(resident_details.flat_no)
    .bind(resident_details.building.to_string())
    .bind(resident_details.society.to_string())
    .bind(otp.to_string())
    .execute(&data.db)
    .await;

    match save_visitor_query_result {
        Ok(_) => {
            return (
                axum::http::StatusCode::OK,
                "Visitor saved successfully"
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't save visitor {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                "WHOOPS"
            ).into_response();
        }
    }
}

pub async fn get_recent_visitor_otp(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {

    let query = format!("SELECT otp FROM visitors WHERE host_email = {:?} ORDER BY visitor_id DESC", headers.get("email").unwrap());

    let query_result = sqlx::query(&query)
        .fetch_one(&data.db)
        .await;
    
        match query_result {
            Ok(visitor) => {
                return(
                    axum::http::StatusCode::OK,
                    Json(json!({
                        "otp": visitor.try_get::<String, _>("otp").unwrap_or_default()
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Couldn't read data {}", err);
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    "WHOOPS"
                ).into_response();
            }
        };
}