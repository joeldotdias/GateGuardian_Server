use std::sync::Arc;
use rand::Rng;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json
};

use serde_json::json;
use sqlx::{ Row, query, query_as };

use crate::{
    config::AppState,
    error::GGError,
    middleware::CurrUser,
    resident::schema::{
        AddHomeDetailsSchema, AdminResidentDto, AdminSecurityDto, DashProfileDetails,
        NoticeDto, RegularDto, ResidentProfileDto, SaveNoticeSchema, SaveRegularSchema,
        SaveVisitorSchema, UpdatePfpParams, UpdateResidentProfileSchema, VisitorResidentDto
    }
};


pub async fn get_resident_by_email(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

    let society_id_query = query("SELECT society_id FROM users WHERE email = ?")
        .bind(&email);

    let society_id = match society_id_query
        .fetch_one(&data.db)
        .await {
            Ok(resident) => {
                resident.try_get::<i32, _>("society_id").unwrap()
            }
            Err(err) => {
                eprintln!("Could not find society: {}", err);
                return GGError::DefunctCredentials("Your society could not be found").into_response();
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
                (StatusCode::OK, Json(resident)).into_response()
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                GGError::ServerError("Could not fetch resident data").into_response()
            }
    }
}

// Profile
pub async fn add_resident_home_details(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<AddHomeDetailsSchema>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            (
                StatusCode::OK,
                Json(json!({
                    "message": "Home details updated successfully"
                }))
            ).into_response()
        }
        Err(err) => {
            eprintln!("Could not update profile: {}", err);
            GGError::Stupidity("Could not update home details").into_response()
        }
    }
}

pub async fn update_resident_profile(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<UpdateResidentProfileSchema>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            eprintln!("Could not update profile: {}", err);
            GGError::Stupidity("Could not update profile").into_response()
        }
    }
}

pub async fn update_resident_pfp(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<UpdatePfpParams>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            (
                StatusCode::OK,
                Json(json!({
                    "message": "Pfp updated successfully"
                }))
            ).into_response()
        }
        Err(err) => {
            eprintln!("err: {}", err);
            GGError::Stupidity("Could not update pfp").into_response()
        }
    }
}


//Dashboard
pub async fn get_dashboard_details(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

    let profile_details_query = query_as::<_, DashProfileDetails>("
        SELECT u.name, r.flat_no, r.building, r.pfp_url
        FROM residents r NATURAL JOIN users u
        WHERE email = ?;
    ")
    .bind(&email)
    .fetch_one(&data.db);

    match profile_details_query.await {
        Ok(profile) => {
            (
                StatusCode::OK,
                Json(profile)
            ).into_response()
        },
        Err(err) => {
            eprintln!("Could not get dashboard details: {}", err);
            GGError::DefunctCredentials("Could not get dashboard details").into_response()
        }
    }
}


// Visitors
pub async fn get_visitors(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            eprintln!("Couldn't read resident data {}", err);
            return GGError::DefunctCredentials("Could not find resident details").into_response();
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
            (
                StatusCode::OK,
                Json(visitors)
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't read data {}", err);
            GGError::Stupidity("Could not find visitors").into_response()
        }
    }
}

pub async fn save_visitor(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<SaveVisitorSchema>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            eprintln!("Couldn't read resident data {}", err);
            return GGError::DefunctCredentials("Could not find resident details").into_response();
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
                (
                    StatusCode::OK,
                    Json(json!({
                        "msg": "Visitor saved successfully"
                    }))
                ).into_response()
            }
            Err(err) => {
                eprintln!("Couldn't save visitor {}", err);
                GGError::ServerError("Could not save visitor details").into_response()
            }
    }
}

pub async fn get_recent_visitor_otp(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            eprintln!("Couldn't read resident data {}", err);
            return GGError::DefunctCredentials("Could not find resident details").into_response();
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
            (
                StatusCode::OK,
                Json(json!({
                    "code": visitor.try_get::<String, _>("code").unwrap_or_default()
                }))
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't read visitor data {}", err);
            GGError::DefunctCredentials("Could not find visitor code").into_response()
        }
    }
}

pub async fn get_regulars(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {
    let email = curr_user.email;

    let get_regulars_query = query_as::<_, RegularDto>("
        SELECT r.name, r.role, r.entry, r.code
        FROM regulars r
        WHERE r.resident_email = ?
    ")
    .bind(email);

    let regulars_query_result = get_regulars_query
        .fetch_all(&data.db)
        .await;

    match regulars_query_result {
        Ok(regulars) => {
            (
                StatusCode::OK,
                Json(regulars)
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't get regulars data {}", err);
            GGError::Stupidity("Could not find regular details").into_response()
        }
    }
}

pub async fn save_regular(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<SaveRegularSchema>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            eprintln!("Couldn't read resident data {}", err);
            return GGError::DefunctCredentials("Could not find resident details").into_response();
        }
    };

    let code = rand::thread_rng().gen_range(100000..=999999);

    let save_regular_query = query(r#"
        INSERT INTO regulars (society_id, name, role, entry, resident_email, code)
        VALUES (?, ?, ?, ?, ?, ?)
    "#)
    .bind(society_id)
    .bind(payload.name)
    .bind(payload.role)
    .bind(payload.entry)
    .bind(&email)
    .bind(code.to_string());

    match save_regular_query
        .execute(&data.db)
        .await {
            Ok(_) => {
                (
                    StatusCode::OK,
                    Json(json!({
                        "msg": "Regular saved successfully"
                    }))
                ).into_response()
            }
            Err(err) => {
                eprintln!("Couldn't save regular {}", err);
                GGError::ServerError("Could not save regular details").into_response()
            }
    }
}

pub async fn get_recent_regular_otp(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
) -> impl IntoResponse {

    let email = curr_user.email;

    let get_code_query = query("
        SELECT code FROM regulars
        WHERE resident_email = ?
        ORDER BY regular_id DESC
    ")
    .bind(email);

    let get_code_query_result = get_code_query
        .fetch_one(&data.db)
        .await;

    match get_code_query_result {
        Ok(regular) => {
            (
                StatusCode::OK,
                Json(json!({
                    "code": regular.try_get::<String, _>("code").unwrap_or_default()
                }))
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't read data {}", err);
            GGError::Stupidity("Could not find regular details").into_response()
        }
    }
}


// Admin
pub async fn get_residents_by_society(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            (
                StatusCode::OK,
                Json(residents)
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't get residents data {}", err);
            GGError::Stupidity("Could not fetch resident details").into_response()
        }
    }
}

pub async fn get_security_by_society(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            (
                StatusCode::OK,
                Json(securities)
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't get securities data {}", err);
            GGError::Stupidity("Could not fetch security details").into_response()
        }
    }
}


// Notices
pub async fn get_notices(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>
) -> impl IntoResponse {

    let email = curr_user.email;

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
            (
                StatusCode::OK,
                Json(notices)
            ).into_response()
        }
        Err(err) => {
            eprintln!("Couldn't get notices data {}", err);
            return GGError::ServerError("Could not find notice details").into_response();
        }
    }
}

pub async fn add_notice(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<SaveNoticeSchema>
) -> impl IntoResponse{

    let email = curr_user.email;

    let get_society_query = query("SELECT society_id FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(&data.db)
        .await;

    let society_id = match get_society_query {
            Ok(resident) => {
                resident.try_get::<i32, _>("society_id").unwrap()
            }
            Err(err) => {
                eprintln!("Could not find society: {}", err);
                return GGError::DefunctCredentials("Could not find your society details").into_response();
            }
        };

    let add_notice_query = query("
        INSERT INTO notices (title, body, category, society_id) VALUES (
            ?, ?, ?, ?
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
                (
                    StatusCode::CREATED,
                    Json(json!({
                        "msg": "Notice added"
                    }))
                ).into_response()
            }
            Err(err) => {
                eprintln!("Failed to add notice: {:?}", err);
                GGError::ServerError("Could not add notice").into_response()
            }
    }
}
