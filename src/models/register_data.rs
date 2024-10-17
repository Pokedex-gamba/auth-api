use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
}
