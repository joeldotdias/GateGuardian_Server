use serde::{ Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Deserialize, Debug)]
pub struct AddHomeDetailsSchema {
    pub flat: i64,
    pub building: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdateResidentProfileSchema {
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


#[derive(Serialize, Debug, FromRow)]
pub struct VisitorResidentDto {
    #[serde(rename="visitorId")]
    pub visitor_id: i32,
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String,
    #[serde(rename="hostEmail")]
    pub host_email: String,
    pub otp: String
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct SaveVisitorSchema {
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String,
    #[serde(rename="hostEmail")]
    pub host_email: String
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ResidentDetailsSchema {
    pub flat_no: i32,
    pub building: String,
    pub society: String
}


#[derive(Deserialize, Debug)]
pub struct SavePersonSchema {
    pub name: String,
    pub email: String
}