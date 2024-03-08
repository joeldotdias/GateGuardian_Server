use chrono::DateTime;
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct SecurityProfileDto {
    pub name: String,
    #[serde(rename= "badgeId")]
    pub badge_id: String,
    #[serde(rename= "phoneNo")]
    pub phone_no: String,
    #[serde(rename= "pfpUrl")]
    pub pfp_url: Option<String>,
    pub society: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct VisitorSecurityDto {
    #[serde(rename="visitorId")]
    pub visitor_id: i32,
    pub name: String,
    #[serde(rename="hostFlat")]
    pub host_flat: i32,
    #[serde(rename="hostBuilding")]
    pub host_building: String,
    pub society: String,
    pub code: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct VisitorLogDto {
    pub name: String,
    #[serde(rename="hostFlat")]
    pub host_flat: i32,
    #[serde(rename="hostBuilding")]
    pub host_building: String,
    pub entry: DateTime<chrono::Utc>
}

#[derive(Debug, Deserialize)]
pub struct VerifiedVisitorParams {
    #[serde(rename="visitorId")]
    pub visitor_id: i32
}

#[derive(Debug, FromRow)]
pub struct VerifiedVisitorDetails {
    pub name: String,
    pub phone_no: String,
    pub resident_id: i32,
}


#[derive(Debug, Deserialize)]
pub struct UpdateSecurityProfileSchema {
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String
}

#[derive(Debug, Deserialize)]
pub struct UpdatePfpParams {
    #[serde(rename="pfpUrl")]
    pub pfp_url: String
}