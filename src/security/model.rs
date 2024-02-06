use sqlx::FromRow;
use chrono::DateTime;

#[derive(Debug, FromRow)]
pub struct Security {
    pub security_id: i32,
    pub email: String,
    pub phone_no: String,
    pub badge_id: String,
    pub pfp_url: String
}

#[derive(Debug, FromRow)]
pub struct VisitorLog {
    pub log_id: i32,
    pub name: String,
    pub phone_no: String,
    pub resident_id: i32,
    pub entry: DateTime<chrono::Utc>
}