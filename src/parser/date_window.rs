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
    pub end: i32,
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
            .transpose()?
            .unwrap_or_else(|| time::OffsetDateTime::now_utc().to_julian_day());

        if begin > end {
            return Err(DateWindowError::InvalidDate(
                "begin can not be greater than end".to_string(),
            ));
        }

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
    use std::str::FromStr;

    use super::DateWindow;

    #[test]
    fn absolute_snippets() {
        let snippets = ["2002:jan:1", "2002:2:2,2003:1:1"];

        let today = time::OffsetDateTime::now_utc();

        let half_open = DateWindow::from_str(snippets[0]).unwrap();
        assert_eq!(half_open.end, today.to_julian_day());
        assert_eq!(half_open.begin, 2452276);

        let closed = DateWindow::from_str(snippets[1]).unwrap();
        assert_eq!(closed.begin, 2452308);
        assert_eq!(closed.end, 2452641);
    }

    #[test]
    fn relative_snippets() {
        let snippets = ["jan:1,feb:1", "jan:1"];

        let today = time::OffsetDateTime::now_utc();
        let this_year = today.year();

        let closed = DateWindow::from_str(snippets[0]).unwrap();
        assert_eq!(
            closed.begin,
            time::Date::from_calendar_date(this_year, time::Month::January, 1)
                .unwrap()
                .to_julian_day()
        );
        assert_eq!(
            closed.end,
            time::Date::from_calendar_date(this_year, time::Month::February, 1)
                .unwrap()
                .to_julian_day()
        );

        let half_open = DateWindow::from_str(snippets[1]).unwrap();
        assert_eq!(
            half_open.begin,
            time::Date::from_calendar_date(this_year, time::Month::January, 1)
                .unwrap()
                .to_julian_day()
        );
        assert_eq!(half_open.end, today.to_julian_day());
    }
}
