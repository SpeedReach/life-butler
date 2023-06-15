use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::driver::model::HttpResponse;


#[derive(Deserialize, Debug, ToSchema)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserLoginResponse {
    user_id: String,
}


impl From<String> for UserLoginResponse{
    fn from(value: String) -> Self {
        Self { user_id: value }
    }
}


