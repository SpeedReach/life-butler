use axum::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use futures::StreamExt;
use mongodb::bson::Bson::DateTime;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::options::FindOptions;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::event::EventRepository;


#[async_trait]
pub trait FindRecentEventRepository {
    async fn find_recent_events(&self, user_id: String,skip: u64,count: i64) -> Result<Vec<Event>,Report<DatabaseError>>;
}

#[async_trait]
impl FindRecentEventRepository for EventRepository {
    async fn find_recent_events(&self, user_id: String, skip: u64, count: i64) -> Result<Vec<Event>, Report<DatabaseError>> {
        let collection: Collection<Event> = self.driver.get_database().collection(self.collection_id.as_str());
        let filter = doc! { "end_time": {"$gt": DateTime(bson::DateTime::now())} };
        let options = FindOptions::builder()
            .skip(skip)
            .limit(count)
            .sort(doc! {"start_time": 1})
            .build();
        let mut cursor = collection.find(filter, options).await
            .into_report()
            .change_context(DatabaseError)?;
        let mut events: Vec<Event> = Vec::new();
        while let Some(result) = cursor.next().await {
            let event = result.into_report().change_context(DatabaseError)?;
            events.push(event);
        }
        Ok(events)
    }
}