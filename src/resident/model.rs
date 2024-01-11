use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Resident {
    pub resident_id: u32,
    pub name: String,
    pub email: String,
    pub pfp_url: String,
    pub about_me: String,
    pub phone_no: String,
    pub flat_no: u32,
    pub building: String,
    pub society: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Visitor{
    pub visitor_id: u32,
    pub name: String,
    pub phone_no: String,
    pub host_email: String,
    pub host_flat: u32,
    pub host_building: String,
    pub host_society: String,
    pub uid: String,
    pub otp: String
}
