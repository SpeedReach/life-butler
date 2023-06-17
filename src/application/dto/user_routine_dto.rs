use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;
use crate::infrastructure::database::entities::routine::UserRoutine;
use crate::shared::utils::date_time::utc_to_utc8;

#[derive(Serialize,Clone)]
pub struct UserRoutineDTO {
    pub user_id: String,
    pub sleeping: bool,
    pub last_eaten_dur: i64,
    pub last_wakeup_dur: i64
}


impl From<UserRoutine> for UserRoutineDTO {
    fn from(value: UserRoutine) -> Self {

        Self{
            user_id: value.user_id,
            sleeping: value.sleeping,
            last_eaten_dur: (Utc::now() -value.last_eaten.to_chrono()).num_minutes(),
            last_wakeup_dur: (Utc::now() -value.last_wakeup.to_chrono()).num_minutes(),
        }
    }
}
