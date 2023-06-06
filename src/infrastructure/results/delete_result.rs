

#[derive(Debug, PartialEq, Clone)]
pub enum  DeleteResult {
    NotFound,
    Success(u64)
}


impl DeleteResult{
    pub fn new(base :mongodb::results::DeleteResult)->DeleteResult{
        if base.deleted_count == 0{
            return DeleteResult::NotFound;
        }
        DeleteResult::Success(base.deleted_count)
    }
}