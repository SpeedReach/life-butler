use std::sync::Arc;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::repositories::event::EventRepository;
use crate::infrastructure::repositories::event::insert_event::InsertEventRepository;

pub struct CreateEventUseCase{
    repository: Arc<dyn InsertEventRepository+Sync+Send>
}


impl CreateEventUseCase{

    pub fn new(repository :Arc<EventRepository>) -> Self{
        Self{repository}
    }


}