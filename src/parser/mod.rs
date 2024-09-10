pub mod schedule;
pub mod walker;

mod time_span;
mod utils;

use pest::Parser;
use pest_derive::Parser;
use zemen::Zemen;

use crate::error::{self, Result};
use crate::parser::schedule::Calender;
use crate::Schedule;

#[derive(Parser)]
#[grammar = "parser/quex.pest"]
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

        let (time, mut description) = utils::get_time_description(&mut schedule)
            .map_err(error::invalid_format(loc, schedule_line.clone()))?;

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
                    utils::month_from_quex(month.as_str()),
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
                    utils::werh_from_quex(month.as_str()),
                    day.as_str().parse().unwrap(),
                )
                .map_err(error::invalid_format(loc, schedule_line))?;

                schedules.push(Schedule::new(
                    description,
                    Calender::from(schedule_date),
                    time,
                ));
            }
            _ => unreachable!(),
        }
    }

    Ok(schedules)
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

        let input = r#"2016 neh 1, in ethiopia
2024 mar 1, sample description.
d=5, recurring monthly
1992* feb 29, reacurring yeal: year: \y and past_time: \a"#;

        let output = vec![
            super::Schedule {
                description: "in ethiopia".to_string(),
                date: Calender::from(Zemen::from_eth_cal(2016, zemen::Werh::Nehase, 1).unwrap()),
                time: None,
                diff: None,
            },
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
                date: Calender::from(
                    Date::from_calendar_date(2024, time::Month::July.next(), 5).unwrap(),
                ),
                time: None,
                diff: None,
            },
            Schedule {
                description: "reacurring yeal: year: 1992 and past_time: 32".to_string(),
                date: Calender::from(
                    Date::from_calendar_date(1992, time::Month::February, 29).unwrap(),
                ),
                time: None,
                diff: None,
            },
        ];

        let schedules = super::parse_quex(input).unwrap();
        assert_eq!(schedules, output);
    }
}
