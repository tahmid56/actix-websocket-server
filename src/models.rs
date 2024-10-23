use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiRequest {
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}