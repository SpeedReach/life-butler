use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use super::response::HttpResponse;

#[derive(Deserialize, Debug)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    user_id: String,
}

impl UserLoginResponse {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

impl IntoResponse for HttpResponse<UserLoginResponse> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
