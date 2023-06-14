use std::fmt::Debug;

use axum::{response::IntoResponse, Json};
use error_stack::Context;
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum None {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HttpResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub is_success: bool,
}

impl<T> HttpResponse<T> {
    pub fn success(data: T, message: String) -> Self {
        Self {
            data: Some(data),
            message,
            is_success: true,
        }
    }

    pub fn fail(context: String) -> Self {
        Self {
            data: None,
            message: context,
            is_success: false,
        }
    }
}
