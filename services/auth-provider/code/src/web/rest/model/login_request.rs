use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 5, max = 100), email)]
    pub email: String,

    #[validate(length(min = 5, max = 100))]
    pub password: String,
}
