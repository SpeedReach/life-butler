use std::error::Error;
use std::fmt::{Display, Write};
use error_stack::{Context, Report};

#[derive(Debug, Clone)]
pub struct DatabaseError;

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A database error occurred ")
    }
}

impl Context for DatabaseError {

}