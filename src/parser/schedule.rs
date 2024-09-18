use serde_derive::Serialize;
use std::fmt;

use crate::calender::Calender;
use crate::parser::time_span;

impl Schedule {
    pub fn new(description: String, date: Calender, time: Option<time_span::TimeSpan>) -> Self {
        Schedule {
            description,
            date,
            time,
            diff: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Schedule {
    pub description: String,
    pub date: Calender,
    pub time: Option<time_span::TimeSpan>,
    pub diff: Option<i32>,
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = self
            .time
            .as_ref()
            .map(|t| format!("{t}, "))
            .unwrap_or("".to_string());

        write!(f, "{}, {}{}", self.date, time, self.description)
    }
}
