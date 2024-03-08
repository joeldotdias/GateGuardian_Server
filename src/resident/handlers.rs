use std::sync::Arc;
use rand::Rng;

use axum::{
    extract::State,
    http::{
        header::HeaderMap,
        StatusCode
    },
    response::IntoResponse,
    Json
};

use serde_json::json;
use sqlx::{ Row, query, query_as };

use crate::{
    config::AppState,
    resident::schema::{
        ResidentProfileDto, DashProfileDetails,
        AddHomeDetailsSchema,    UpdatePfpParams, UpdateResidentProfileSchema, 
        SaveVisitorSchema, VisitorResidentDto, SaveNoticeSchema, NoticeDto,
        RegularDto, SaveRegularSchema,
        AdminResidentDto, AdminSecurityDto,
    },
    sanitize_headers
};


// App entry
pub async fn get_resident_by_email(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {

    let email = match sanitize_headers(headers, "email") {
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

    println!("{}", email);

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
    
    let get_resident_query = query_as::<_, ResidentProfileDto>("
        SELECT u.name, r.pfp_url, r.about_me, r.phone_no, r.flat_no, r.building, s.society_name AS society 
        FROM users u, residents r, societies s
        WHERE r.email=? AND u.email=? AND s.society_id=? 
    ")
    .bind(&email)
    .bind(&email)
    .bind(society_id);

    match get_resident_query
        .fetch_one(&data.db)
        .await {
            Ok(resident) => {
                return (StatusCode::OK, Json(resident)).into_response();
            }
            Err(err) => {
                dbg!("Error: {}", err);
                return(
                    StatusCode::INTERNAL_SERVER_ERROR,
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

    let email = match sanitize_headers(headers, "email"){
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

    let update_details_query = query("
            UPDATE residents
            SET flat_no = ?, building = ?
            WHERE email = ?
        ")
        .bind(payload.flat)
        .bind(payload.building)
        .bind(email);

    let query_result = update_details_query
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "message": "Home details updated successfully"
                }))
            ).into_response();
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            return (
                StatusCode::BAD_REQUEST,
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
    
    let email = match sanitize_headers(headers, "email"){
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
            UPDATE residents
            SET about_me = ?, phone_no = ?
            WHERE email = ?
        ")
        .bind(payload.about_me)
        .bind(payload.phone_no)
        .bind(email);

    let query_result = update_profile_query
        .execute(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            (
                StatusCode::OK,
                Json(json!({
                    "message": "Profile updated successfully"
                }))
            ).into_response()
        }
        Err(err) => {
            dbg!("Could not update profile: {}", err);
            (
                StatusCode::BAD_REQUEST,
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

    let email = match sanitize_headers(headers, "email"){
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
            UPDATE residents
            SET pfp_url = ? 
            WHERE email = ?
        ")
        .bind(payload.pfp_url)
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


//Dashboard
pub async fn get_dashboard_details(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let email = match sanitize_headers(headers, "email") {
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

    let profile_details_query = query_as::<_, DashProfileDetails>("
        SELECT u.name, r.flat_no, r.building, r.pfp_url 
        FROM residents r NATURAL JOIN users u 
        WHERE email = ?;
    ")
    .bind(&email)
    .fetch_one(&data.db);

    match profile_details_query.await {
        Ok(profile) => {
            return(
                StatusCode::OK,
                Json(profile)
            ).into_response();
        },
        Err(err) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "err": err.to_string()
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

    let email = match sanitize_headers(headers, "email"){
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

    let resident_id_query = query("SELECT resident_id FROM residents WHERE email = ?")
        .bind(email);
    
    let resident_id_query_result = resident_id_query
        .fetch_one(&data.db)
        .await;
    
    let resident_id = match resident_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("resident_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };
    
    let get_visitors_query = query_as::<_, VisitorResidentDto>("
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
        Ok(visitors) => {
            return(
                StatusCode::OK,
                Json(visitors)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't read data {}", err);
            return (
                StatusCode::BAD_REQUEST,
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

    let email = match sanitize_headers(headers, "email"){
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

    let resident_id_query = query("SELECT resident_id FROM residents WHERE email = ?")
        .bind(email);
    
    let resident_id_query_result = resident_id_query
        .fetch_one(&data.db)
        .await;
    
    let resident_id = match resident_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("resident_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };

    let code = rand::thread_rng().gen_range(100000..=999999);

    let save_visitor_query = query(r#"
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
                    StatusCode::OK,
                    Json(json!({
                        "msg": "Visitor saved successfully"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Couldn't save visitor {}", err);
                return (
                    StatusCode::BAD_REQUEST,
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

    let email = match sanitize_headers(headers, "email"){
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

    let resident_id_query = query("SELECT resident_id FROM residents WHERE email = ?")
        .bind(email);
    
    let resident_id_query_result = resident_id_query
        .fetch_one(&data.db)
        .await;
    
    let resident_id = match resident_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("resident_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };
    
    let get_code_query = query("
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
                    StatusCode::OK,
                    Json(json!({
                        "code": visitor.try_get::<String, _>("code").unwrap_or_default()
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Couldn't read data {}", err);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "err": "Could not find visitor details"
                    }))
                ).into_response();
            }
        };
}

pub async fn get_regulars(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    let email = match sanitize_headers(headers, "email"){
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

    let get_regulars_query = query_as::<_, RegularDto>("
        SELECT r.name, r.role, r.entry, r.departure, r.code
        FROM regulars r 
        WHERE r.resident_email = ?
    ")
    .bind(email);

    let regulars_query_result = get_regulars_query
        .fetch_all(&data.db)
        .await;

    match regulars_query_result {
        Ok(regulars) => {
            return (
                StatusCode::OK,
                Json(regulars)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get regulars data {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not fetch regulars details"
                }))
            ).into_response();
        }
    }
}

pub async fn save_regular(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SaveRegularSchema>
) -> impl IntoResponse {

    let email = match sanitize_headers(headers, "email"){
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
    
    let society_id_query_result = society_id_query
        .fetch_one(&data.db)
        .await;
    
    let society_id = match society_id_query_result {
        Ok(resident) => {
            resident.try_get::<i32, _>("society_id").unwrap()
        }
        Err(err) => {
            dbg!("Couldn't read resident data {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "err": "Could not find resident details"
                }))
            ).into_response();
        }
    };

    let code = rand::thread_rng().gen_range(100000..=999999);

    let save_regular_query = query(r#"
        INSERT INTO regulars (society_id, name, role, entry, departure, resident_email, code)
        VALUES (?, ?, ?, ?, ?, ?, ?)
    "#)
    .bind(society_id)
    .bind(payload.name)
    .bind(payload.role)
    .bind(payload.entry)
    .bind(payload.departure)
    .bind(&email)
    .bind(code.to_string());
    
    match save_regular_query
        .execute(&data.db)
        .await {
            Ok(_) => {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "msg": "Regular saved successfully"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Couldn't save regular {}", err);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "err": "Could not save regular details"
                    }))
                ).into_response();
            }
    }
}


// Admin
pub async fn get_residents_by_society(
    State(data): State<Arc<AppState>>,
    headers: HeaderMap
) -> impl IntoResponse {
    
    let email = match sanitize_headers(headers, "admin"){
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

    let get_residents_query = query_as::<_, AdminResidentDto>("
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
                StatusCode::OK,
                Json(residents)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get residents data {}", err);
            return (
                StatusCode::BAD_REQUEST,
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

    let email = match sanitize_headers(headers, "admin"){
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

    let get_securities_query = query_as::<_, AdminSecurityDto>("
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
                StatusCode::OK,
                Json(securities)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get securities data {}", err);
            return (
                StatusCode::BAD_REQUEST,
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
   
    let email = match sanitize_headers(headers, "email"){
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

   let get_notices_query = query_as::<_, NoticeDto>("
        SELECT title, body, category, posted
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
                StatusCode::OK,
                Json(notices)
            ).into_response();
        }
        Err(err) => {
            dbg!("Couldn't get notices data {}", err);
            return (
                StatusCode::BAD_REQUEST,
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
    
    let email = match sanitize_headers(headers, "admin"){
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

    let get_society_query = query("SELECT society_id FROM users WHERE email = ?")
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
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "err": "Your society could not be found"
                        }))
                    ).into_response();
            }
        };

    let add_notice_query = query("
        INSERT INTO notices (title, body, category, society_id) VALUES (
            ?, ?, ?
        )
    ")
        .bind(payload.title)
        .bind(payload.body)
        .bind(payload.category)
        .bind(society_id);

    match add_notice_query
        .execute(&data.db)
        .await{
            Ok(_) => {
                return (
                    StatusCode::CREATED,
                    Json(json!({
                        "msg": "Notice added"
                    }))
                ).into_response();
            }
            Err(err) => {
                dbg!("Failed to add notice: {:?}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "err": "Failed to add notice"
                    }))
                ).into_response();
            }
    };
}
