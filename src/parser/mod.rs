pub mod walker;

use std::fmt;

use pest::Parser;
use pest_derive::Parser;
use time::Date;
use zemen::Zemen;

use crate::error::{self, Result};

#[enum_dispatch::enum_dispatch]
trait DisplayDate {
    fn date(&self) -> (i32, String, u8);
}

pub trait JulianDayNumber {
    fn julian_day(&self) -> i32;
}

#[enum_dispatch::enum_dispatch(DisplayDate)]
#[derive(Debug, PartialEq)]
pub enum Calender {
    Date,
    Zemen,
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

#[derive(Debug, PartialEq)]
pub struct Schedule {
    pub description: String,
    pub date: Calender,
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.date, self.description)
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
    pub fn new(description: String, date: Calender) -> Self {
        Schedule { description, date }
    }
}

#[derive(Parser)]
#[grammar = "parser/quex.pest"]
pub struct QuexParser;

fn parse_quex(raw_quex: &str) -> Result<Vec<Schedule>> {
    let mut schedules = vec![];
    let schedule_list = QuexParser::parse(Rule::schedule_list, raw_quex).map_err(error::qerror)?;
    let today: time::Date = time::OffsetDateTime::now_utc().date();

    for schedule in schedule_list {
        let loc = schedule.line_col();
        let r = schedule.as_str().to_string();

        let mut schedule = schedule.into_inner();

        let Some(date) = schedule.next() else {
            continue;
        };
        let mut description = schedule.next().unwrap().as_str().to_string(); // won't fail

        match date.as_rule() {
            Rule::gregorian_date => {
                let mut gregorian_date = date.into_inner();

                let year = gregorian_date.next().unwrap(); // won't fail
                let mut year_str = year.as_str();
                let is_named_year = year.as_rule() == Rule::named_yearly;

                if is_named_year {
                    let today: time::Date = time::OffsetDateTime::now_utc().date();

                    year_str = year.as_str().strip_suffix('*').unwrap(); // won't fail
                    let years_past = today.year() - year_str.parse::<i32>().unwrap(); // won't
                                                                                      // fail

                    description = description
                        .replace("\\y", year_str)
                        .replace("\\a", &years_past.to_string());
                }

                let month = gregorian_date.next().unwrap();
                let day = gregorian_date.next().unwrap();

                let schedule_date = time::Date::from_calendar_date(
                    // TODO: check the current month and only report the future
                    // bithday year.
                    year_str
                        .parse()
                        .map(|year| {
                            if year < today.year() && is_named_year {
                                return today.year();
                            }
                            return year;
                        })
                        .unwrap_or(today.year()),
                    month_from_quex(month.as_str()),
                    day.as_str().parse().unwrap(),
                )
                .map_err(error::invalid_date(loc, r))?;

                schedules.push(Schedule::new(description, Calender::from(schedule_date)));
            }
            Rule::recurring_monthly => {
                let raw_date = date;

                let day = raw_date
                    .as_str()
                    .strip_prefix("d=")
                    .and_then(|n| n.parse::<u8>().ok())
                    .unwrap();

                let mut month = today.month();

                if day < today.day() {
                    month = today.month().next();
                }

                let schedule_date = time::Date::from_calendar_date(today.year(), month, day)
                    .map_err(error::invalid_date(loc, r))?;

                schedules.push(Schedule::new(description, Calender::from(schedule_date)));
            }
            // TODO: instead of using ethiopian_date or recurring_monthly, or use gregorian_date
            // make a generic error that has the line, line number, and column number,
            // when errors happen on converting dates.
            Rule::ethiopian_date => {
                let mut ethiopian_date = date.into_inner();

                let year = ethiopian_date.next().unwrap(); // won't fail
                let mut year_str = year.as_str();

                if year.as_rule() == Rule::named_yearly {
                    let today = Zemen::today();
                    year_str = year.as_str().strip_suffix('*').unwrap(); // won't fail
                    let years_past = today.year() - year_str.parse::<i32>().unwrap(); // won't fail

                    description = description
                        .replace("\\y", year_str)
                        .replace("\\a", &years_past.to_string());
                }

                let month = ethiopian_date.next().unwrap(); // won't fail
                let day = ethiopian_date.next().unwrap(); // won't fail

                let schedule_date = Zemen::from_eth_cal(
                    year_str.parse().unwrap(),
                    werh_from_quex(month.as_str()),
                    day.as_str().parse().unwrap(),
                )
                .map_err(error::invalid_date(loc, r))?;

                schedules.push(Schedule::new(description, Calender::from(schedule_date)));
            }
            _ => unreachable!(),
        }
    }

    Ok(schedules)
}

fn werh_from_quex(as_str: &str) -> zemen::Werh {
    match as_str {
        "mes" | "መስከ" => zemen::Werh::Meskerem,
        "tik" | "ጥቅም" => zemen::Werh::Tikimit,
        "hed" | "ህዳር" => zemen::Werh::Hedar,
        "tah" | "ታኅሣ" => zemen::Werh::Tahasass,
        "tir" | "ጥር" => zemen::Werh::Tir,
        "yek" | "የካቲ" => zemen::Werh::Yekatit,
        "meg" | "መጋቢ" => zemen::Werh::Megabit,
        "miy" | "ሚያዝ" => zemen::Werh::Miyazia,
        "gin" | "ግንቦ" => zemen::Werh::Ginbot,
        "sen" | "ሴኒ" => zemen::Werh::Sene,
        "ham" | "ሐምሌ" => zemen::Werh::Hamle,
        "neh" | "ነሐሴ" => zemen::Werh::Nehase,
        "pua" | "ጳጉሜ" => zemen::Werh::Puagme,
        _ => unreachable!(),
    }
}

fn month_from_quex(month: &str) -> time::Month {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_quex() {
        use super::Calender;
        use super::Schedule;
        use time::Date;
        use zemen::Zemen;

        time::Date::from_calendar_date(2024, time::Month::March, 1).unwrap();

        let input = "2016 neh 1, in ethiopia\n2024 mar 1, sample description.\nd=5, recurring monthly\n1992* feb 29, reacurring yeal: year: \\y and past_time: \\a\n";
        let output = vec![
            super::Schedule {
                description: "in ethiopia".to_string(),
                date: Calender::from(Zemen::from_eth_cal(2016, zemen::Werh::Nehase, 1).unwrap()),
            },
            Schedule {
                description: "sample description.".to_string(),
                date: Calender::from(
                    Date::from_calendar_date(2024, time::Month::March, 1).unwrap(),
                ),
            },
            Schedule {
                description: "recurring monthly".to_string(),
                date: Calender::from(Date::from_calendar_date(2024, time::Month::July, 5).unwrap()),
            },
            Schedule {
                description: "reacurring yeal: year: 1992 and past_time: 32".to_string(),
                date: Calender::from(
                    Date::from_calendar_date(1992, time::Month::February, 29).unwrap(),
                ),
            },
        ];

        let schedules = super::parse_quex(input).unwrap();
        assert_eq!(schedules, output);
    }
}
