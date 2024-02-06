use serde::{ Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct ResidentProfileDto {
    pub name: String,
    #[serde(rename= "pfpUrl")]
    pub pfp_url: Option<String>,
    #[serde(rename= "aboutMe")]
    pub about_me: Option<String>,
    #[serde(rename= "phoneNo")]
    pub phone_no: Option<String>,
    #[serde(rename= "flatNo")]
    pub flat_no: Option<i32>,
    pub building: Option<String>,
    pub society: String
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
pub struct VisitorResidentDto {
    #[serde(rename="visitorId")]
    pub visitor_id: i32,
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String,
    #[serde(rename="hostEmail")]
    pub host_email: String,
    pub code: String
}

#[derive(Debug, FromRow, Deserialize, )]
pub struct SaveVisitorSchema {
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String
}


#[derive(Debug, Deserialize)]
pub struct SavePersonSchema {
    pub name: String,
    pub email: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AdminResidentDto {
    pub name: String,
    pub email: String,
    #[serde(rename= "flatNo")]
    pub flat_no: i32,
    pub building: String
}

#[derive(Debug, FromRow, Serialize)]
pub struct AdminSecurityDto {
    pub name: String,
    pub email: String,
    #[serde(rename= "badgeId")]
    pub badge_id: String
}