use serde_derive::Serialize;

use crate::calender::{DateInfo, Event};

#[derive(Debug, PartialEq, Serialize)]
pub struct Schedule {
    pub description: String,
    #[serde(skip_serializing)]
    pub julian_day_number: i32,
    pub diff: i32,
    pub date: String,
}

impl<T: DateInfo> From<Event<T>> for Schedule {
    fn from(event: Event<T>) -> Self {
        let today = time::OffsetDateTime::now_utc();
        let date = event.date.julian_day();

        Self {
            description: event.message,
            julian_day_number: date,
            diff: date - today.to_julian_day(),
            date: event.date.pretty_print(),
        }
    }
}
