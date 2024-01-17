use serde::{ Serialize, Deserialize};
use sqlx::FromRow;

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