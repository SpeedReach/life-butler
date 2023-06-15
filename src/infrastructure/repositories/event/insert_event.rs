use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::results::insert_result::InsertResult;
use axum::async_trait;
use error_stack::{IntoReport, Report};
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::event::EventRepository;

#[async_trait]
pub trait InsertEventRepository {
    async fn insert_event(&self, user: Event) -> Result<InsertResult<Event>,Report<DatabaseError>>;
}

#[async_trait]
impl InsertEventRepository for EventRepository {

    async fn insert_event(&self, event: Event) -> Result<InsertResult<Event>, Report<DatabaseError>> {

        let collection = self.get_collection();

        let result = collection.insert_one(&event, None).await
            .into_report();

        match result {
            Ok(r) => Ok(InsertResult::Success(event)),
            Err(err) => Err(err.change_context(DatabaseError))
        }
    }
}