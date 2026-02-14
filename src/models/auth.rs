use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub refresh_token: Option<String>,
}
