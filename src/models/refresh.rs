use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshPayload {
    pub grant_type: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub id_token: String,
    pub refresh_token: String,
}
