

#[derive(Clone, Debug, PartialEq)]
pub enum CreateResult<T>{
    AlreadyExists(String),
    Success(T)
}

