use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEventRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
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
