#[cfg(feature = "eth")]
pub mod eth;
#[cfg(feature = "eth")]
use zemen::Zemen;

use std::fmt;

use time::Date;

trait DisplayPlain {
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl DisplayPlain for time::Date {
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.month(), self.day(), self.year())
    }
}

#[derive(Debug, PartialEq)]
pub enum Calender {
    Date(Date),

    #[cfg(feature = "eth")]
    Zemen(Zemen),
}

impl Calender {
    pub fn julian_day(&self) -> i32 {
        match self {
            Calender::Date(d) => d.to_julian_day(),

            #[cfg(feature = "eth")]
            Calender::Zemen(d) => d.to_jdn(),
        }
    }
}

impl From<Date> for Calender {
    fn from(value: Date) -> Self {
        Calender::Date(value)
    }
}

impl fmt::Display for Calender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Calender::Date(d) => d.display(f),

            #[cfg(feature = "eth")]
            Calender::Zemen(d) => d.display(f),
        }
    }
}

impl serde::Serialize for Calender {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

pub fn month_from_quex(month: &str) -> time::Month {
    match month {
        "jan" => time::Month::January,
        "feb" => time::Month::February,
        "mar" => time::Month::March,
        "apr" => time::Month::April,
        "may" => time::Month::May,
        "jun" => time::Month::June,
        "jul" => time::Month::July,
        "aug" => time::Month::August,
        "sep" => time::Month::September,
        "oct" => time::Month::October,
        "nov" => time::Month::November,
        "dec" => time::Month::December,
        _ => unreachable!(),
    }
}
