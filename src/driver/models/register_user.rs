use crate::driver;
use crate::infrastructure::database::entities::user::User;
use axum::response::IntoResponse;
use axum::Json;
use axum::{http::StatusCode, response::Response};
use error_stack::Context;
use serde::{Deserialize, Serialize};

use super::response::HttpResponse;

#[derive(Deserialize, Debug)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}

impl IntoResponse for HttpResponse<User> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
