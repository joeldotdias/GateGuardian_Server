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
    security::schema::{
            SecurityProfileDto, UpdateSecurityProfileSchema, UpdatePfpParams,
            VisitorSecurityDto, VerifiedVisitorParams, VerifiedVisitorDetails, VisitorLogDto
        }
};


// App entry
pub async fn get_security_by_email(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    let email = headers.get("email").unwrap().to_str().unwrap();

    let society_id_query = format!("SELECT society_id FROM users WHERE email = {:?}", email);

    let society_id = match sqlx::query(&society_id_query)
        .fetch_one(&data.db)
        .await {
            Ok(resident) => {
                resident.try_get::<i32, _>("society_id").unwrap()
            }
            Err(err) => {
                dbg!("Could not find society: {}", err);
                    return (
                        axum::http::StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "err": "Your society could not be found"
                        }))
                    ).into_response();
            }
    };

    let get_security_query = sqlx::query_as::<_, SecurityProfileDto>("
        SELECT u.name, s.badge_id, s.phone_no, s.pfp_url, soc.society_name AS society 
        FROM users u, securities s, societies soc 
        WHERE s.email = ? AND u.email = ? AND soc.society_id = ?;
    ")
    .bind(email)
    .bind(email)
    .bind(society_id);
    
    match get_security_query
        .fetch_one(&data.db)
        .await {
            Ok(security) => {
                return (axum::http::StatusCode::OK, Json(security)).into_response();
            }
            Err(err) => {
                dbg!("Error: {}", err);
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
    
    let email = headers.get("email").unwrap();
    
    let security_society_query = format!("SELECT society_id from users WHERE email = {:?}", email);

    let security_society = match sqlx::query(&security_society_query)
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<i32, _>("society_id").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{} {}", err, security_society_query);
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let get_visitors_query = sqlx::query_as::<_, VisitorSecurityDto>("
    SELECT v.visitor_id, v.name, r.flat_no AS host_flat, r.building AS host_building, soc.society_name AS society, v.code
    FROM visitors v, residents r, societies soc
    WHERE r.email IN (
        SELECT r.email 
        FROM residents r NATURAL JOIN users u 
        WHERE u.society_id = ?
    ) AND v.resident_id = r.resident_id AND soc.society_id = ?
    ")
    .bind(security_society)
    .bind(security_society);

    let query_result = get_visitors_query
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
    Json(payload): Json<VerifiedVisitorParams>
) -> impl IntoResponse {
    
    let visitor_id = payload.visitor_id;

    let get_visitor_data_query = sqlx::query_as::<_, VerifiedVisitorDetails>("
        SELECT v.name, v.phone_no, v.resident_id
        FROM visitors v
        WHERE v.visitor_id = ?
    ")
    .bind(visitor_id);
    
    let visitor_data = match get_visitor_data_query
        .fetch_one(&data.db)
        .await {
            Ok(visitor_data) => visitor_data,
            Err(err) => {
                dbg!("Could not fetch visitor data:{}", err);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "err": "Could not fetch visitor data with provided details"
                    }))
                ).into_response();
            }
    };
    
    let remove_from_visitors_query = sqlx::query("
            DELETE FROM visitors
            WHERE visitor_id = ?
        ")
        .bind(visitor_id);
    
    let remove_visitor_result = remove_from_visitors_query
        .execute(&data.db)
        .await;

    match remove_visitor_result {
        Err(err) => {
            dbg!("Could not remove visitor :{}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not remove visitor"
                }))
            ).into_response();
        }
        _ => {}
    }

    let add_visitor_to_logs_query = sqlx::query("
        INSERT INTO visitor_logs (name, phone_no, resident_id)
        VALUES (?, ?, ?)
    ")
    .bind(visitor_data.name)
    .bind(visitor_data.phone_no)
    .bind(visitor_data.resident_id);
    
    match add_visitor_to_logs_query
        .execute(&data.db)
        .await {
            Ok(_) => {
                return (
                    axum::http::StatusCode::OK,
                    Json(json!({
                        "msg": "Successfully moved visitor to logs"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Could not move visitor to logs: {}", err);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "err": "Could not move visitor to logs"
                    }))
                ).into_response();
            }
    }
}

pub async fn get_visitor_logs(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let email = headers.get("email").unwrap();
    
    let security_society_query = format!("SELECT society_id from users WHERE email = {:?}", email);

    let security_society = match sqlx::query(&security_society_query)
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<i32, _>("society_id").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{} {}", err, security_society_query);
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let get_visitor_logs_query = sqlx::query_as::<_, VisitorLogDto>("
        SELECT vl.log_id, vl.name, r.flat_no as host_flat, r.building as host_building, soc.society_name AS host_society, vl.entry
        FROM visitor_logs vl, residents r, societies soc
        WHERE r.email IN (
            SELECT r.email 
            FROM residents r NATURAL JOIN users u 
            WHERE u.society_id = ?
        ) AND vl.resident_id = r.resident_id AND soc.society_id = ?;
    ")
    .bind(security_society)
    .bind(security_society);

    let query_result = get_visitor_logs_query
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
            dbg!("Could not fetch visitor logs data: {}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch visitor logs data"
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
        ", params.pfp_url.to_string(), headers.get("email").unwrap());
    
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