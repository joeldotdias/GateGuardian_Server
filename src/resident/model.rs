use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Resident {
    pub resident_id: i32,
    pub name: String,
    pub email: String,
    #[serde(rename= "pfpUrl")]
    pub pfp_url: String,
    #[serde(rename= "aboutMe")]
    pub about_me: String,
    #[serde(rename= "phoneNo")]
    pub phone_no: String,
    #[serde(rename= "flatNo")]
    pub flat_no: i32,
    pub building: String,
    pub society: String
}