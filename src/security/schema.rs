use chrono::DateTime;
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct SecurityProfileDto {
    name: String,
    #[serde(rename= "badgeId")]
    badge_id: String,
    #[serde(rename= "phoneNo")]
    phone_no: String,
    #[serde(rename= "pfpUrl")]
    pfp_url: Option<String>,
    society: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct VisitorSecurityDto {
    #[serde(rename="visitorId")]
    visitor_id: i32,
    name: String,
    #[serde(rename="hostFlat")]
    host_flat: i32,
    #[serde(rename="hostBuilding")]
    host_building: String,
    society: String,
    code: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct VisitorLogDto {
    name: String,
    #[serde(rename="hostFlat")]
    host_flat: i32,
    #[serde(rename="hostBuilding")]
    host_building: String,
    entry: DateTime<chrono::Utc>
}

#[derive(Debug, Deserialize)]
pub struct VerifiedVisitorParams {
    #[serde(rename="visitorId")]
    pub visitor_id: i32
}

#[derive(Debug, Deserialize)]
pub struct NotifyParams {
    #[serde(rename = "flatNo")]
    pub flat_no: String,
    pub building: String
}

#[derive(Debug, FromRow)]
pub struct VerifiedVisitorDetails {
    pub name: String,
    pub phone_no: String,
    pub resident_id: i32,
}


#[derive(Debug, FromRow, Serialize)]
pub struct ResidentDetails {
    name: String,
    #[serde(rename = "phoneNo")]
    phone_no: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct SecurityRegularDto {
    name: String,
    role: String,
    entry: String,
    code: String
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