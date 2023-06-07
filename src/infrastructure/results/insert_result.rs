

#[derive(Clone, Debug, PartialEq)]
pub enum InsertResult<T>{
    AlreadyExists(String),
    Success(T)
}

