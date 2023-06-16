pub mod register_user;
pub mod user_login;

use std::collections::HashMap;
use std::fmt::Debug;
use axum::{response::IntoResponse, Json};
use axum::response::Response;
use error_stack::Context;
use futures::future::err;
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};
use utoipa::openapi::{ObjectBuilder, RefOr, Schema};
use utoipa::openapi::SchemaType::Boolean;
use crate::application::{OperationErr, OperationError};

#[derive(Serialize)]
pub struct Empty;

#[derive(Serialize)]
pub struct HttpResponse<T>{
    title: String,
    is_success: bool,
    data: Option<T>,
    error: Option<OperationErr>
}

impl<T> HttpResponse<T>{
    pub fn success(data: T, title: impl Into<String>) -> Self{
        Self{
            title: title.into(),
            is_success: true,
            data: Some(data),
            error: None
        }
    }
    
    pub fn success_empty(title: impl Into<String>) -> Self{
        Self{
            title: title.into(),
            is_success: true,
            data: None,
            error: None
        }
    }
    
    pub fn fail<E: OperationError>(title: impl Into<String>,error: E)-> Self{
        Self{
            title: title.into(),
            is_success: false,
            data: None,
            error: Some(OperationErr(Box::new(error)))
        }
    }
}


impl <T: Serialize> IntoResponse for HttpResponse<T>{
    fn into_response(self) -> Response {
        if self.is_success {
            (StatusCode::OK,Json(self)).into_response()
        }
        else {
            match &self.error {
                None => (StatusCode::INTERNAL_SERVER_ERROR,Json(self)).into_response(),
                Some(err) => (StatusCode::from_u16(err.0.status_code()).unwrap(),Json(self)).into_response()
            }
        }
    }
}