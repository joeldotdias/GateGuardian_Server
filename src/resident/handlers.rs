use std::sync::Arc;
use rand::Rng;

use axum::{
    extract::State,
    http::header::HeaderMap,
    response::IntoResponse,
    Json
};

use serde_json::json;
use sqlx::Row;

use crate::{
    AppState,
    resident::schema::{
        ResidentProfileDto, AddHomeDetailsSchema, UpdateResidentProfileSchema, UpdatePfpParams,
        VisitorResidentDto, SaveVisitorSchema,
        SaveNoticeSchema,
        AdminResidentDto, AdminSecurityDto
    }
};

use super::schema::NoticeDto;



// App entry
pub async fn get_resident_by_email(
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
    
    let get_resident_query = sqlx::query_as::<_, ResidentProfileDto>("
        SELECT u.name, r.pfp_url, r.about_me, r.phone_no, r.flat_no, r.building, s.society_name AS society 
        FROM users u, residents r, societies s
        WHERE r.email=? AND u.email=? AND s.society_id=? 
    ")
    .bind(email)
    .bind(email)
    .bind(society_id);

    match get_resident_query
        .fetch_one(&data.db)
        .await {
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

    let email = headers.get("email").unwrap();

    let query = format!("
            UPDATE residents
            SET flat_no = {}, building = '{}'
            WHERE email = {:?}
        ", payload.flat, payload.building, email);

    let query_result = sqlx::query(&query)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            return (
                axum::http::StatusCode::OK,
                Json(json!({
                    "message": "Home details updated successfully"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not update home details"
                }))
            ).into_response();
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
            SET about_me = '{}', phone_no = '{}'
            WHERE email = {:?}
        ", payload.about_me, payload.phone_no, headers.get("email").unwrap());

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
    Json(payload): Json<UpdatePfpParams>
) -> impl IntoResponse {

    let query = format!("
            UPDATE residents
            SET pfp_url = '{}' 
            WHERE email = {:?}
        ", payload.pfp_url.to_string(), headers.get("email").unwrap());
    
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


// Visitors
pub async fn get_visitors(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    let email = headers.get("email").unwrap();

    let resident_id_query = format!("SELECT resident_id FROM residents WHERE email = {:?}", email);
    
    let resident_id_query_result = sqlx::query(&resident_id_query)
        .fetch_one(&data.db)
        .await;
    
    let resident_id = match resident_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("resident_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };
    
    let get_visitors_query = sqlx::query_as::<_, VisitorResidentDto>("
        SELECT v.visitor_id, v.name, v.phone_no, r.email as host_email, v.code
        FROM visitors v, residents r
        WHERE v.resident_id = ? AND r.resident_id = ?;
    ")
    .bind(resident_id)
    .bind(resident_id);
    
    let query_result = get_visitors_query
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
                Json(json!({
                    "err": "Could not find visitors"
                }))
            ).into_response();
        }
    };
}

pub async fn save_visitor(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SaveVisitorSchema>
) -> impl IntoResponse {

    let email = headers.get("email").unwrap();

    let resident_id_query = format!("SELECT resident_id FROM residents WHERE email = {:?}", email);
    
    let resident_id_query_result = sqlx::query(&resident_id_query)
        .fetch_one(&data.db)
        .await;
    
    let resident_id = match resident_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("resident_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };

    let code = rand::thread_rng().gen_range(100000..=999999);

    let save_visitor_query = sqlx::query(r#"
        INSERT INTO visitors (name, phone_no, resident_id, code)
        VALUES (?, ?, ?, ?)
    "#)
    .bind(payload.name)
    .bind(payload.phone_no)
    .bind(resident_id)
    .bind(code.to_string());

    match save_visitor_query
        .execute(&data.db)
        .await {
            Ok(_) => {
                return (
                    axum::http::StatusCode::OK,
                    Json(json!({
                        "msg": "Visitor saved successfully"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Couldn't save visitor {}", err);
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(json!({
                        "err": "Could not save visitor details"
                    }))
                ).into_response();
            }
    }
}

pub async fn get_recent_visitor_otp(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {

    let email = headers.get("email").unwrap();

    let resident_id_query = format!("SELECT resident_id FROM residents WHERE email = {:?}", email);
    
    let resident_id_query_result = sqlx::query(&resident_id_query)
        .fetch_one(&data.db)
        .await;
    
    let resident_id = match resident_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("resident_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };
    
    let get_code_query = sqlx::query("
        SELECT code FROM visitors
        WHERE resident_id = ?
        ORDER BY visitor_id DESC
    ")
    .bind(resident_id);

    let get_code_query_result = get_code_query
        .fetch_one(&data.db)
        .await;
    
        match get_code_query_result {
            Ok(visitor) => {
                return(
                    axum::http::StatusCode::OK,
                    Json(json!({
                        "code": visitor.try_get::<String, _>("code").unwrap_or_default()
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Couldn't read data {}", err);
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(json!({
                        "err": "Could not find visitor details"
                    }))
                ).into_response();
            }
        };
}


// Admin
pub async fn get_residents_by_society(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let email = headers.get("admin").unwrap().to_str().unwrap();

    let get_residents_query = sqlx::query_as::<_, AdminResidentDto>("
        SELECT u.name, u.email, r.flat_no, r.building 
        FROM residents r NATURAL JOIN users u 
        WHERE u.society_id = (
            SELECT u.society_id 
            FROM users u 
            WHERE u.email = ?
        )
    ")
    .bind(email);
    
    let get_residents_query_result = get_residents_query
        .fetch_all(&data.db)
        .await;

    match get_residents_query_result {
        Ok(residents) => {
            return (
                axum::http::StatusCode::OK,
                Json(residents)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get residents data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not fetch resident details"
                }))
            ).into_response();
        }
    }
}

pub async fn get_security_by_society(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    let email = headers.get("admin").unwrap().to_str().unwrap();

    let get_securities_query = sqlx::query_as::<_, AdminSecurityDto>("
        SELECT u.name, u.email, s.badge_id 
        FROM securities s NATURAL JOIN users u 
        WHERE u.society_id = (
            SELECT u.society_id 
            FROM users u 
            WHERE u.email = ?
        )
    ")
    .bind(email);
    
    let get_securities_query_result = get_securities_query
        .fetch_all(&data.db)
        .await;

    match get_securities_query_result {
        Ok(securities) => {
            return (
                axum::http::StatusCode::OK,
                Json(securities)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get securities data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not fetch securities details"
                }))
            ).into_response();
        }
    }
}


// Notices
pub async fn get_notices(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
   let email = headers.get("email").unwrap().to_str().unwrap();

   let get_notices_query = sqlx::query_as::<_, NoticeDto>("
        SELECT title, body, posted 
        FROM notices
        WHERE society_id = (
            SELECT society_id 
            FROM users
            WHERE email = ?
        )
    ")
       .bind(email);

   match get_notices_query
       .fetch_all(&data.db)
       .await  {
        Ok(notices) => {
            return (
                axum::http::StatusCode::OK,
                Json(notices)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get notices data {}", err);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not fetch notices details"
                }))
            ).into_response();
        }   
    };
}

pub async fn add_notice(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SaveNoticeSchema>
) -> impl IntoResponse{
    let email = headers.get("admin").unwrap().to_str().unwrap();

    let get_society_query = sqlx::query("SELECT society_id FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(&data.db)
        .await;

    let society_id = match get_society_query {
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

    let add_notice_query = sqlx::query("
        INSERT INTO notices (title, body, society_id) VALUES (
            ?, ?, ?
        )
    ")
        .bind(payload.title)
        .bind(payload.body)
        .bind(society_id);

    match add_notice_query
        .execute(&data.db)
        .await{
            Ok(_) => {
                return (
                    axum::http::StatusCode::CREATED,
                    Json(json!({
                        "msg": "Notice added"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Failed to add notice: {:?}", err);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "err": "Failed to add notice"
                    }))
                ).into_response();
            }
    };
}
