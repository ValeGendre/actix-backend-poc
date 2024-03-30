use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserInput {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub administrator: Option<bool>,
}