use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterData {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub password: String,
}
