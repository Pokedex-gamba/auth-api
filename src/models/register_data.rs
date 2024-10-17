use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
pub struct RegisterData {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub password: String,
}
