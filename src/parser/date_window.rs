use core::fmt;
use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;

use crate::calender;

#[derive(Parser)]
#[grammar = "parser/grammer/quex.pest"]
#[grammar = "parser/grammer/date_window.pest"]
pub struct DateRangeParser;

#[derive(Clone, Debug)]
pub struct DateWindow {
    pub begin: i32,
    pub end: Option<i32>,
}

#[derive(thiserror::Error, Debug)]
pub enum DateWindowError {
    ParseError(String),
    EmptyField,
    InvalidDate(String),
}

impl fmt::Display for DateWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateWindowError::ParseError(s) => writeln!(f, "{}", s),
            DateWindowError::EmptyField => writeln!(f, "You must provide an input"),
            DateWindowError::InvalidDate(s) => writeln!(f, "{}", s),
        }
    }
}

mod error {
    use super::{DateWindowError, Rule};

    pub fn parse_error(err: pest::error::Error<Rule>) -> DateWindowError {
        DateWindowError::ParseError(err.to_string())
    }

    pub fn invalid_date(err: time::error::ComponentRange) -> DateWindowError {
        DateWindowError::InvalidDate(err.to_string())
    }
}

impl FromStr for DateWindow {
    type Err = DateWindowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_window_rule = DateRangeParser::parse(Rule::date_window, s)
            .map_err(error::parse_error)?
            .next()
            .ok_or(DateWindowError::EmptyField)?;

        let mut beg_end = date_window_rule.into_inner();

        let begin = rule_to_jdn(beg_end.next().map(|r| r.into_inner()).unwrap())?; // won't fail
        let end = beg_end
            .next()
            .map(|r| r.into_inner())
            .map(rule_to_jdn)
            .transpose()?;

        Ok(DateWindow { begin, end })
    }
}

fn rule_to_jdn(rule: pest::iterators::Pairs<Rule>) -> Result<i32, DateWindowError> {
    let mut year = None;
    let mut month = None;
    let mut day = None;

    for r in rule {
        match r.as_rule() {
            Rule::year => year = Some(r.as_str().parse().expect("unable to parse year")),
            Rule::gregorian_month => month = Some(calender::month_from_quex(r.as_str())),
            Rule::month => {
                month = Some(
                    // This unwrap might be susceptible to panic, if an integer
                    // overflow occuers at that point you should ask your self
                    // what are you doing with all these months
                    time::Month::try_from(r.as_str().parse::<u8>().unwrap())
                        .map_err(error::invalid_date)?,
                )
            }
            Rule::day => day = Some(r.as_str().parse().expect("unable to parse day")),
            _ => unreachable!(),
        }
    }

    let year = year.unwrap_or_else(|| time::OffsetDateTime::now_utc().date().year());
    let month = month.unwrap(); // guaranteed by pest
    let day = day.unwrap(); // guaranteed by pest

    Ok(time::Date::from_calendar_date(year, month, day)
        .map_err(error::invalid_date)?
        .to_julian_day())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_full_field() {}
}
