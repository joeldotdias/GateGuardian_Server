use serde::{ Serialize, Deserialize};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ParamOptions {
    pub pfp_url: String,
    pub flat: String,
    pub building: String,
    pub name: String,
    pub aboutMe: String,
    pub phoneNo: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdatePfpParams {
    pub pfp_url: String
}