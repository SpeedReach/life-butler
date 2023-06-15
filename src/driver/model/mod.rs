pub mod create_event;
pub mod register_user;
pub mod user_login;

use std::fmt::Debug;
use axum::{response::IntoResponse, Json};
use axum::response::Response;
use error_stack::Context;
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};
use utoipa::openapi::{ObjectBuilder, RefOr, Schema};
use utoipa::openapi::SchemaType::Boolean;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum None {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq )]
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


impl<'s, T> ToSchema<'s> for HttpResponse<T> where T: ToSchema<'s> {

    fn schema() -> (&'s str, RefOr<Schema>) {
        let  schema = ObjectBuilder::new()
            .property("data", T::schema().1)
            .property("is_success", bool::schema())
            .property("message", String::schema())
            .property("title", String::schema())
            .build();

        ("HttpResponse", RefOr::from(Schema::from(schema)))
    }
}