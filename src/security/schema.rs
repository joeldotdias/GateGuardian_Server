use serde::{ Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct VisitorSecurityDto {
    #[serde(rename="visitorId")]
    pub visitor_id: i32,
    pub name: String,
    #[serde(rename="hostFlat")]
    pub host_flat: i32,
    #[serde(rename="hostBuilding")]
    pub host_building: String,
    #[serde(rename="society")]
    pub host_society: String,
    pub otp: String
}

#[derive(Deserialize, Debug)]
pub struct VerifiedVisitorParams {
    #[serde(rename="visitorId")]
    pub visitor_id: String
}

#[derive(FromRow, Debug)]
pub struct VerifiedVisitorDetails {
    pub name: String,
    pub phone_no: String,
    pub host_flat: i32,
    pub host_building: String,
    pub host_society: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateSecurityProfileSchema {
    pub name: String,
    #[serde(rename="aboutMe")]
    pub about_me: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdatePfpParams {
    pub pfpUrl: String
}