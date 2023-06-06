use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::infrastructure::database::entities::user::User;

#[derive(Deserialize,Debug)]
pub struct RegisterUserRequest{
    pub email: String,
    pub password: String
}

#[derive(Serialize, Debug)]
pub struct RegisterUseResponse(pub User);


impl IntoResponse for RegisterUseResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK,Json(self.0)).into_response()
    }
}
