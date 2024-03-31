use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub category: String
}

#[derive(Debug, Deserialize)]
pub struct GetUserParams {
    pub email: String,
}
