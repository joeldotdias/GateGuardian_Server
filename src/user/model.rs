use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub category: String,
    pub society: String
}