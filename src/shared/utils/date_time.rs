use chrono::{DateTime, FixedOffset, Timelike, TimeZone, Utc};
use serde::{Serialize, Serializer};

static TW_OFFSET: Option<FixedOffset> = FixedOffset::east_opt(3600 * 8);



fn now() -> DateTime<FixedOffset>{
    let tz: FixedOffset = TimeZone::from_offset(&TW_OFFSET.unwrap());
    Utc::now().with_timezone(&tz)
        .with_nanosecond(0).unwrap()
}