use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub category: String,
    pub society: String
}