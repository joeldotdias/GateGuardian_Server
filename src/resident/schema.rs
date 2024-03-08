use serde::{ Serialize, Deserialize};
use sqlx::FromRow;
use chrono::DateTime;

#[derive(Debug, FromRow, Serialize)]
pub struct ResidentProfileDto {
    name: String,
    #[serde(rename= "pfpUrl")]
    pfp_url: Option<String>,
    #[serde(rename= "aboutMe")]
    about_me: Option<String>,
    #[serde(rename= "phoneNo")]
    phone_no: Option<String>,
    #[serde(rename= "flatNo")]
    flat_no: Option<i32>,
    building: Option<String>,
    society: String
}

#[derive(Debug, Deserialize)]
pub struct AddHomeDetailsSchema {
    pub flat: i32,
    pub building: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateResidentProfileSchema {
    #[serde(rename="aboutMe")]
    pub about_me: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String
}

#[derive(Debug, Deserialize)]
pub struct UpdatePfpParams {
    #[serde(rename="pfpUrl")]
    pub pfp_url: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct DashProfileDetails {
    name: String,
    #[serde(rename="flatNo")]
    flat_no: i32,
    building: String,
    #[serde(rename="pfpUrl")]
    pfp_url: String
}


#[derive(Debug, FromRow, Serialize)]
pub struct VisitorResidentDto {
    #[serde(rename="visitorId")]
    visitor_id: i32,
    name: String,
    #[serde(rename="phoneNo")]
    phone_no: String,
    #[serde(rename="hostEmail")]
    host_email: String,
    code: String
}

#[derive(Debug, FromRow, Deserialize, )]
pub struct SaveVisitorSchema {
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String
}

#[derive(Debug, Deserialize)]
pub struct SaveNoticeSchema {
    pub title: String,
    pub body: String,
    pub category: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct NoticeDto {
    title: String,
    body: String,
    category: String,
    posted: DateTime<chrono::Utc>
}


#[derive(Debug, Deserialize)]
pub struct SavePersonSchema {
    pub name: String,
    pub email: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct AdminResidentDto {
    name: String,
    email: String,
    #[serde(rename= "flatNo")]
    flat_no: Option<i32>,
    building: Option<String>
}

#[derive(Debug, FromRow, Serialize)]
pub struct AdminSecurityDto {
    name: String,
    email: String,
    #[serde(rename= "badgeId")]
    badge_id: Option<String>
}

#[derive(Debug, FromRow, Serialize)]
pub struct RegularDto {
    name: String,
    role: String,
    entry: String,
    code: String
}

#[derive(Debug, Deserialize)]
pub struct SaveRegularSchema {
    pub name: String,
    pub role: String,
    pub entry: String
}
