use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
}
