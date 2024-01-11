use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub society: String,
    pub category: String
}

#[derive(Deserialize, Debug)]
pub struct GetUserParams {
    pub email: String
}