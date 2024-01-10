use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub society: String,
    pub category: String
}