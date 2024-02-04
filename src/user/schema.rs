use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub category: String
}

#[derive(Deserialize, Debug)]
pub struct GetUserParams {
    pub email: String
}