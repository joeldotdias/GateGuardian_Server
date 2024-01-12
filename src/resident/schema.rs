use serde::{ Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct SaveResidentHomeSchema {
    pub flat: i64,
    pub building: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdateResidentProfileSchema {
    pub name: String,
    pub aboutMe: String,
    pub phoneNo: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdatePfpParams {
    pub pfpUrl: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SavePersonSchema {
    pub name: String,
    pub email: String
}