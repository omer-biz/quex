use pest::Parser as _;
use pest_derive::Parser;
use time::Date;

use super::{ColumnLocation, DateInfo, DateResult, Event, LineError};

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

impl DateInfo for Date {
    fn julian_day(&self) -> i32 {
        self.to_julian_day()
    }

    fn pretty_print(&self) -> String {
        format!("{} {}, {}", self.month(), self.day(), self.year())
    }
}

#[derive(Parser)]
#[grammar = "parser/grammar/base.pest"]
#[grammar = "parser/grammar/gre.pest"]
pub struct GreQuexParser;

pub fn parse_gre_quex(line: &str) -> DateResult<impl DateInfo> {
    let schedule = match GreQuexParser::parse(Rule::schedule, line) {
        Ok(s) => s,
        Err(e) => {
            // TODO: if you encounter any funny bugs, check here
            if e.variant.message() == "expected schedule" {
                return Ok(None);
            }
            return Err(LineError::ParsingError {
                error: e.to_string(),
                message: e.variant.message().to_string(),
                column: ColumnLocation::new(e.location),
            });
        }
    };

    let today = time::OffsetDateTime::now_utc();
    let mut schedule = schedule.into_iter().next().unwrap().into_inner();

    match schedule.peek().unwrap().as_rule() {
        Rule::gregorian_date => {
            let mut date = schedule.next().unwrap().into_inner();
            let mut message = schedule.next().unwrap().as_str().to_string();

            let month = month_from_quex(date.next().unwrap().as_str());
            let day =
                date.next().unwrap().as_str().parse::<u8>().map_err(|_| {
                    LineError::InvalidValue("day can't be greater than 31".to_string())
                })?;

            let year = date.next().unwrap();

            let year = if year.as_rule() == Rule::named_yearly {
                let year = year.into_inner().next().unwrap();
                let yearn: i32 = year.as_str().parse().unwrap();
                let year_past = today.year() - yearn;

                message = message
                    .replace("\\y", year.as_str())
                    .replace("\\a", year_past.to_string().as_str());

                if yearn < today.year() {
                    today.year()
                } else {
                    yearn
                }
            } else if year.as_rule() == Rule::yearly {
                today.year()
            } else {
                year.as_str().parse::<i32>().unwrap()
            };

            let date = match Date::from_calendar_date(year, month, day) {
                Ok(d) => d,
                Err(e) => return Err(LineError::InvalidValue(e.to_string())),
            };

            Ok(Some(Event::new(date, message)))
        }
        Rule::recurring_monthly => {
            let day = schedule
                .next()
                .unwrap()
                .into_inner()
                .as_str()
                .parse()
                .unwrap();

            let mut month = today.month();

            if day < today.day() {
                month = month.next();
            }

            let date = match Date::from_calendar_date(today.year(), month, day) {
                Ok(d) => d,
                Err(e) => return Err(LineError::InvalidValue(e.to_string())),
            };

            let message = schedule.next().unwrap().as_str().to_string();

            Ok(Some(Event::new(date, message)))
        }
        _ => unreachable!(),
    }
}
