use std::sync::Arc;

use axum::{
    extract::{State, Query },
    http::{
        header::HeaderMap,
        StatusCode
    },
    response::IntoResponse,
    Json
};
use serde_json::json;
use sqlx::{ query, query_as, Row };

use crate::{
    config::AppState,
    sanitize_headers,
    security::schema::{
            SecurityProfileDto, SecurityRegularDto, UpdatePfpParams, ResidentDetails,
            UpdateSecurityProfileSchema, VerifiedVisitorDetails, VerifiedVisitorParams, VisitorLogDto, VisitorSecurityDto
        }
};

use super::schema::OOParams;


// App entry
pub async fn get_security_by_email(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };
    
    let society_id_query = query("SELECT society_id FROM users WHERE email = ?")
        .bind(&email);

    let society_id = match society_id_query
        .fetch_one(&data.db)
        .await {
            Ok(resident) => {
                resident.try_get::<i32, _>("society_id").unwrap()
            }
            Err(err) => {
                dbg!("Could not find society: {}", err);
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "err": "Your society could not be found"
                        }))
                    ).into_response();
            }
    };

    let get_security_query = query_as::<_, SecurityProfileDto>("
        SELECT u.name, s.badge_id, s.phone_no, s.pfp_url, soc.society_name AS society 
        FROM users u, securities s, societies soc 
        WHERE s.email = ? AND u.email = ? AND soc.society_id = ?;
    ")
    .bind(&email)
    .bind(&email)
    .bind(society_id);
    
    match get_security_query
        .fetch_one(&data.db)
        .await {
            Ok(security) => {
                return (StatusCode::OK, Json(security)).into_response();
            }
            Err(err) => {
                dbg!("Error: {}", err);
                return(
                    StatusCode::INTERNAL_SERVER_ERROR,
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
    
    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };
    
    let security_society_query = query("SELECT society_id from users WHERE email = ?")
        .bind(email);

    let security_society = match security_society_query
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<i32, _>("society_id").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{}", err);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let get_visitors_query = query_as::<_, VisitorSecurityDto>("
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
                StatusCode::OK,
                Json(visitors)
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch visitors data: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
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

    let get_visitor_data_query = query_as::<_, VerifiedVisitorDetails>("
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
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "err": "Could not fetch visitor data with provided details"
                    }))
                ).into_response();
            }
    };
    
    let remove_from_visitors_query = query("
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
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not remove visitor"
                }))
            ).into_response();
        }
        _ => {}
    }

    let add_visitor_to_logs_query = query("
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
                    StatusCode::OK,
                    Json(json!({
                        "msg": "Successfully moved visitor to logs"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Could not move visitor to logs: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
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
    
    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };
    
    let security_society_query = query("SELECT society_id from users WHERE email = ?")
        .bind(email);

    let security_society = match security_society_query
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<i32, _>("society_id").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{}", err);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let get_visitor_logs_query = query_as::<_, VisitorLogDto>("
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
                StatusCode::OK,
                Json(visitor_logs)
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch visitor logs data: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch visitor logs data"
                }))
            ).into_response();
        }
    }
}



// Notify
pub async fn get_residents_to_notify(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<OOParams>
) -> impl IntoResponse {
    
    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };

    let security_society_query = query("SELECT society_id from users WHERE email = ?")
        .bind(email);

    let security_society = match security_society_query
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<i32, _>("society_id").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{}", err);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };

    let resident_details_query = query_as::<_, ResidentDetails>("
        SELECT u.name, r.phone_no
        FROM users u NATURAL JOIN residents r
        WHERE r.flat_no = ? AND r.building = ? AND society_id = ?
    ")
    .bind(params.flat_no)
    .bind(params.building)
    .bind(security_society);

    let resident_details_result = resident_details_query
        .fetch_all(&data.db)
        .await;

    match resident_details_result {
        Ok(residents) => {
            return (
                StatusCode::OK,
                Json(residents)
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch residents data: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch residents data"
                }))
            ).into_response();
        }
    }
}


// Regulars
pub async fn get_regulars(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };
    
    let security_society_query = query("SELECT society_id from users WHERE email = ?")
        .bind(email);

    let security_society = match security_society_query
        .fetch_one(&data.db)
        .await {
        Ok(society) => society.try_get::<i32, _>("society_id").unwrap_or_default(),
        Err(err) => {
            dbg!("Could not fetch security details :{}", err);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": "Did not find security credentials"
                }))
            ).into_response();
        }
    };
    
    let get_visitor_logs_query = query_as::<_, SecurityRegularDto>("
        SELECT r.name,r.role, r.entry, r.code 
        FROM regulars r 
        WHERE r.society_id = ?
    ")
    .bind(security_society);

    let query_result = get_visitor_logs_query
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(regulars) => {
            return (
                StatusCode::OK,
                Json(regulars)
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not fetch regulars data: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "err": "Could not fetch regulars data"
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
    
    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };
    
    let update_profile_query = query("
            UPDATE securities
            SET name = ?, phone_no = ?
            WHERE email = ?
        ")
        .bind(payload.name)
        .bind(payload.phone_no)
        .bind(email);

    let query_result = update_profile_query
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "message": "Profile updated successfully"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update profile"
                }))
            ).into_response();
        }
    }
}

pub async fn update_security_pfp(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<UpdatePfpParams>
) -> impl IntoResponse {

    let email =  match sanitize_headers(headers, "email") {
        Ok(header) => header,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": err
                }))
            ).into_response();
        }   
    };
    
    let update_pfp_query = query("
            UPDATE securities
            SET pfp_url = ?
            WHERE email = ?
        ")
        .bind(payload.pfp_url.to_string())
        .bind(email);
    
    let query_result = update_pfp_query
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "message": "Pfp updated successfully"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("err: {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update pfp"
                }))
            ).into_response();
        }
    }
}