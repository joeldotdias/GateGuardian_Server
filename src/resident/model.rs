use sqlx::FromRow;
use chrono::DateTime;

#[derive(Debug, FromRow)]
pub struct Resident {
    pub resident_id: i32,
    pub email: String,
    pub pfp_url: String,
    pub about_me: String,
    pub phone_no: String,
    pub flat_no: i32,
    pub building: String,
}

#[derive(Debug, FromRow)]
pub struct Notice {
    pub notice_id: i32,
    pub society_id: i32,
    pub title: String,
    pub body: String,
    pub category: String,
    pub posted: DateTime<chrono::Utc> 
}  

#[derive(Debug, FromRow)]
pub struct Regular {
    pub regular_id: i32,
    pub society_id: i32,
    pub resident_email: String,
    pub name: String,
    pub role: String,
    pub entry: String,
    pub departure: String,
    pub code: String
}
