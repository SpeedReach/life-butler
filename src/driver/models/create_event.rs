use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserLoginResponse {
    user_id: String,
}

impl UserLoginResponse {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

impl IntoResponse for UserLoginResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
