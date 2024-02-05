use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use chrono::DateTime;

// #[derive(Debug, FromRow, Deserialize, Serialize)]
// pub struct Security {
//     pub security_id: i32,
//     pub name: String,
//     pub email: String,
//     #[serde(rename= "pfpUrl")]
//     pub pfp_url: String,
//     #[serde(rename= "phoneNo")]
//     pub phone_no: String,
//     #[serde(rename= "badgeId")]
//     pub badge_id: String,
//     #[serde(rename= "society")]
//     pub society: String
// }

#[derive(Debug, FromRow)]
pub struct Security {
    pub security_id: i32,
    pub email: String,
    pub phone_no: String,
    pub badge_id: String,
    pub pfp_url: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct VisitorLog {
    pub log_id: i32,
    pub name: String,
    #[serde(rename= "phoneNo")]
    pub phone_no: String,
    #[serde(rename= "hostFlat")]
    pub host_flat: i32,
    #[serde(rename= "hostBuilding")]
    pub host_building: String,
    #[serde(rename= "hostSociety")]
    pub host_society: String,
    pub entry: DateTime<chrono::Utc>
}