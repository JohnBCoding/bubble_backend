use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "localId")]
    pub id: String,
    #[serde(rename = "idToken")]
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
    #[serde(rename = "returnSecureToken")]
    pub return_secure_token: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: String,
}
