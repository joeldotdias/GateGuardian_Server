use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub category: String
}
