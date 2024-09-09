use serde_derive::Serialize;
use std::fmt;
use time::Date;
use zemen::Zemen;

use crate::parser::time_span;


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

impl JulianDayNumber for Calender {
    fn julian_day(&self) -> i32 {
        match &self {
            Calender::Date(d) => d.to_julian_day(),
            Calender::Zemen(d) => d.to_jdn(),
        }
    }
}

impl Schedule {
    pub fn new(description: String, date: Calender, time: Option<time_span::TimeSpan>) -> Self {
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
                    "{} {} {}",
                    d.month(),
                    d.day(),
                    d.year()
                )
            }
            Calender::Zemen(d) => {
                write!(
                    f,
                    "{} {} {}",
                    d.month(),
                    d.day(),
                    d.year()
                )
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Calender {
    Date(Date),
    Zemen(Zemen),
}

impl From<Zemen> for Calender {
    fn from(value: zemen::Zemen) -> Self {
        Calender::Zemen(value)
    }
}

impl From<Date> for Calender {
    fn from(value: Date) -> Self {
        Calender::Date(value)
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Schedule {
    pub description: String,
    pub date: Calender,
    pub time: Option<time_span::TimeSpan>,
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
