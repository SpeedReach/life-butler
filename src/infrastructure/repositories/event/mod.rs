use std::sync::Arc;
use mongodb::IndexModel;
use mongodb::options::IndexOptions;
use crate::infrastructure::database::database_service::DatabaseDriver;
use crate::infrastructure::database::entities::event::Event;

pub mod insert_event;
pub mod find_recent_events;

#[derive(Clone)]
pub struct EventRepository{
    driver: Arc<DatabaseDriver>,
    database_id: String,
    collection_id: String
}

impl EventRepository{
    pub async fn new(driver: &Arc<DatabaseDriver>) -> EventRepository {

        Self {
            driver: Arc::clone(driver),
            database_id: driver.get_config().database_id,
            collection_id: String::from("event")
        }
    }


    fn get_collection(&self) -> mongodb::Collection<Event> {
        self.driver
            .get_database()
            .collection::<Event>(&self.collection_id)
    }
    
}