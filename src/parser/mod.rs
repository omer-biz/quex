pub mod date_window;
pub mod schedule;
pub mod walker;

mod time_span;
mod utils;

use pest::Parser;
use pest_derive::Parser;

use crate::calender;
use crate::calender::Calender;
use crate::error::{self, Result};
use crate::Schedule;

#[cfg(feature = "eth")]
use crate::calender::eth;

#[derive(Parser)]
#[grammar = "parser/grammer/quex.pest"]
pub struct QuexParser;

fn parse_quex(raw_quex: &str) -> Result<Vec<Schedule>> {
    let mut schedules = vec![];
    let schedule_list = QuexParser::parse(Rule::schedule_list, raw_quex).map_err(error::qerror)?;
    let today: time::Date = time::OffsetDateTime::now_utc().date();

    for schedule in schedule_list {
        let loc = schedule.line_col();
        let schedule_line = schedule.as_str().to_string();

        let mut schedule = schedule.into_inner();

        let Some(date) = schedule.next() else {
            continue;
        };

        let (time, mut description) = match utils::get_time_description(&mut schedule) {
            Ok(t) => t,
            Err(e) => return Err(error::invalid_format(loc, schedule_line)(e)),
        };

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
                    // birthday year.
                    year_str
                        .parse()
                        .map(|year| {
                            // helpful when printing all the past schedules.
                            // It won't replace their years with the current year.
                            if year < today.year() && is_named_year {
                                return today.year();
                            }
                            year
                        })
                        .unwrap_or(today.year()),
                    calender::month_from_quex(month.as_str()),
                    day.as_str().parse().unwrap(),
                )
                .map_err(error::invalid_format(loc, schedule_line))?;

                schedules.push(Schedule::new(
                    description,
                    Calender::from(schedule_date),
                    time,
                ));
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
                    .map_err(error::invalid_format(loc, schedule_line))?;

                schedules.push(Schedule::new(
                    description,
                    Calender::from(schedule_date),
                    time,
                ));
            }
            // TODO: instead of using ethiopian_date or recurring_monthly, or use gregorian_date
            // make a generic error that has the line, line number, and column number,
            // when errors happen on converting dates.
            Rule::ethiopian_date => {
                #[cfg(feature = "eth")]
                {
                    let ethiopian_date = date.into_inner();
                    let (schedule_date, description) =
                        eth::parse_eth_date(ethiopian_date, description);
                    let schedule_date =
                        schedule_date.map_err(error::invalid_format(loc, schedule_line))?;

                    schedules.push(Schedule::new(
                        description,
                        Calender::from(schedule_date),
                        time,
                    ));
                }
                #[cfg(not(feature = "eth"))]
                {
                    return Err(error::invalid_format(loc, schedule_line)(
                        error::ValueError::EthNotEnabled,
                    ));
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(schedules)
}

#[cfg(test)]
mod tests {
    use super::Calender;
    use super::Schedule;
    use time::Date;

    #[cfg(feature = "eth")]
    use zemen::Zemen;

    #[test]
    fn test_parse_quex() {
        let input = r#"2024 mar 1, sample description.
d=5, recurring monthly
1992* feb 29, reacurring yeal: year: \y and past_time: \a
"#;

        let today = time::OffsetDateTime::now_utc();

        let month = if today.day() > 5 {
            today.month().next()
        } else {
            today.month()
        };

        let output = vec![
            Schedule {
                description: "sample description.".to_string(),
                date: Calender::from(
                    Date::from_calendar_date(2024, time::Month::March, 1).unwrap(),
                ),
                time: None,
                diff: None,
            },
            Schedule {
                description: "recurring monthly".to_string(),
                date: Calender::from(Date::from_calendar_date(today.year(), month, 5).unwrap()),
                time: None,
                diff: None,
            },
            Schedule {
                description: "reacurring yeal: year: 1992 and past_time: 32".to_string(),
                date: Calender::from(
                    Date::from_calendar_date(today.year(), time::Month::February, 29).unwrap(),
                ),
                time: None,
                diff: None,
            },
        ];

        let schedules = super::parse_quex(input).unwrap();
        assert_eq!(schedules, output);
    }

    #[test]
    #[cfg(feature = "eth")]
    fn test_parse_eth() {
        let input = r#"2016 neh 1, in ethiopia
2015* mes 1, another eth  this \y and this \a
"#;

        let today = Zemen::today();

        let output = vec![
            super::Schedule {
                description: "in ethiopia".to_string(),
                date: Calender::from(Zemen::from_eth_cal(2016, zemen::Werh::Nehase, 1).unwrap()),
                time: None,
                diff: None,
            },
            super::Schedule {
                description: "another eth  this 2015 and this 2".to_string(),
                date: Calender::from(
                    Zemen::from_eth_cal(today.year(), zemen::Werh::Meskerem, 1).unwrap(),
                ),
                time: None,
                diff: None,
            },
        ];

        let schedules = super::parse_quex(input).unwrap();
        assert_eq!(schedules, output);
    }
}
