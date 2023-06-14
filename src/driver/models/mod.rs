pub mod create_event;
pub mod register_user;
pub mod user_login;

use std::fmt::Debug;
use axum::{response::IntoResponse, Json};
use axum::response::Response;
use error_stack::Context;
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum None {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HttpResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub title: String,
    pub is_success: bool,
}

impl<T> HttpResponse<T> {
    pub fn success<S: Into<String>> (data: T, title: S) -> Self {
        Self {
            data: Some(data),
            title: title.into(),
            message: String::from(""),
            is_success: true,
        }
    }

    pub fn fail <S1: Into<String>, S2: Into<String>> (title: S1,context: S2) -> Self {
        Self {
            data: None,
            title: title.into(),
            message: context.into(),
            is_success: false,
        }
    }
}

impl <T: Serialize> IntoResponse for HttpResponse<T>{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}