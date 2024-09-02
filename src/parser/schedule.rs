use serde_derive::Serialize;
use std::fmt;
use time::Date;
use zemen::Zemen;

use crate::parser::time_wrapper;

#[enum_dispatch::enum_dispatch]
trait DisplayDate {
    fn date(&self) -> (i32, String, u8);
}

pub trait JulianDayNumber {
    fn julian_day(&self) -> i32;
}

impl serde::Serialize for Calender {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl DisplayDate for Date {
    fn date(&self) -> (i32, String, u8) {
        (self.year(), self.month().to_string(), self.day())
    }
}

impl DisplayDate for Zemen {
    fn date(&self) -> (i32, String, u8) {
        (self.year(), self.month().to_string(), self.day())
    }
}

impl JulianDayNumber for Calender {
    fn julian_day(&self) -> i32 {
        match &self {
            Calender::Date(d) => d.to_julian_day(),
            Calender::Zemen(d) => d.to_jdn(),
        }
    }
}

impl Schedule {
    pub fn new(
        description: String,
        date: Calender,
        time: Option<time_wrapper::TimeWrapper>,
    ) -> Self {
        Schedule {
            description,
            date,
            time,
        }
    }
}

impl fmt::Display for Calender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Calender::Date(d) => {
                write!(
                    f,
                    "{}, {} {} {}",
                    d.weekday().to_string().split_at(3).0,
                    d.month(),
                    d.day(),
                    d.year()
                )
            }
            Calender::Zemen(d) => {
                write!(
                    f,
                    "{}, {} {} {}",
                    d.weekday().short_name(),
                    d.month(),
                    d.day(),
                    d.year()
                )
            }
        }
    }
}

#[enum_dispatch::enum_dispatch(DisplayDate)]
#[derive(Debug, PartialEq)]
pub enum Calender {
    Date,
    Zemen,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Schedule {
    pub description: String,
    pub date: Calender,
    pub time: Option<time_wrapper::TimeWrapper>,
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = self
            .time
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or("".to_string());

        write!(f, "{}, {}: {}", self.date, time, self.description)
    }
}
