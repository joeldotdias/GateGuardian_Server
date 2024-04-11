use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json
};
use serde_json::json;
use sqlx::{ query, query_as, Row };

use crate::{
    config::AppState,
    error::GGError,
    middleware::CurrUser,
    user::{
        model::User,
        schema::CreateUserSchema
    }
};

use super::schema::GetUserParams;

pub async fn get_user(
    State(data): State<Arc<AppState>>,
    Query(params): Query<GetUserParams>
    
) -> impl IntoResponse {
    // println!("{}", curr_user.email);

    let user_query = query_as::<_, User>("
        SELECT u.user_id, u.name, u.email, u.category, soc.society_name AS society 
        FROM users AS u INNER JOIN societies AS soc ON u.society_id = soc.society_id 
        WHERE u.email = ?
    ")
    .bind(params.email)
    .fetch_one(&data.db)
    .await;

    match user_query {
        Ok(user) => {
            (StatusCode::OK, Json(user)).into_response()
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            GGError::DefunctCredentials("Could not fetch user details").into_response()
        }
    }
}

// explicit returns make it way more readable in this fn
#[allow(clippy::needless_return)]
pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Extension(curr_user): Extension<CurrUser>,
    Json(payload): Json<CreateUserSchema>,
) -> impl IntoResponse {

    let name = payload.name.as_str();
    let email = payload.email.as_str();
    let category = payload.category.as_str();
    
    let admin = curr_user.email;

    let society_id_query = query("SELECT society_id FROM users WHERE email = ?")
        .bind(admin);
    
    let society_id = match society_id_query
        .fetch_one(&data.db)
        .await {
            Ok(admin) => {
                admin.try_get::<i32, _>("society_id").unwrap()
            }
            Err(err) => {
                eprintln!("Could not add resident: {}", err);
                    return GGError::DefunctCredentials("Your society has not been registered yet").into_response();
            }
    };
    
    let query_result = query(r#"
        INSERT INTO users (name, email, society_id, category) 
        VALUES (?, ?, ?, ?)
    "#)
    .bind(name)
    .bind(email)
    .bind(society_id)
    .bind(category)
    .execute(&data.db)
    .await;

    if let Err(err) = query_result {
        eprintln!("Could not add to the users table {}", err);
        return GGError::RegistrationFailure("Failed to register user").into_response();
    }
    
    match category {
        "resident" | "admin" => {
            let add_resident_query = query(r#"
                INSERT INTO residents (email) VALUES (?)
            "#)
                .bind(email)
                .execute(&data.db)
                .await;

            match add_resident_query {
                Ok(_) => {
                    return (
                        StatusCode::OK,
                        Json(json!({
                            "msg": "Resident registered successfully"
                        }))
                    ).into_response();
                }
                Err(err) => {
                    eprintln!("Could not add resident: {}", err);
                    return GGError::RegistrationFailure("Failed to register resident").into_response();
                }
            }
        }

        "security" => {
            let add_security_query = query(r#"
                INSERT INTO securities (email) VALUES (?)
            "#)
                .bind(email)
                .execute(&data.db)
                .await;

            match add_security_query {
                Ok(_) => {
                    return (
                        StatusCode::OK,
                        Json(json!({
                            "msg": "Security registered successfully"
                        }))
                    ).into_response();
                }
                Err(err) => {
                    eprintln!("Could not add security: {}", err);
                    return GGError::RegistrationFailure("Failed to register security").into_response();
                }
            }
        }
        _ => {
            return GGError::Stupidity("The specified category did not match any of the available options").into_response();
        }
    }
}
