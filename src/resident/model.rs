use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Resident {
    pub resident_id: i32,
    pub email: String,
    pub pfp_url: String,
    pub about_me: String,
    pub phone_no: String,
    pub flat_no: i32,
    pub building: String,
}