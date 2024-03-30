use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub administrator: bool,
}