use std::fmt::{Display, Formatter, Write};
use axum::response::{IntoResponse, Response};
use error_stack::Context;
use serde::{Serialize, Serializer};

pub mod dto;
pub mod use_case;


pub trait OperationError: Context{
    fn status_code(&self) -> u16;
}


pub struct OperationErr(pub Box<dyn OperationError>);

impl Serialize for OperationErr{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl Display for OperationErr{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl IntoResponse for OperationErr{
    fn into_response(self)->Response{
        self.to_string().into_response()
    }
}
