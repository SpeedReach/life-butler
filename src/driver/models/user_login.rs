use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::driver::models::HttpResponse;


#[derive(Deserialize, Debug)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    user_id: String,
}


impl From<String> for UserLoginResponse{
    fn from(value: String) -> Self {
        Self { user_id: value }
    }
}


