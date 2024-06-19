mod extractor;
mod walker;

use pest::Parser;
use pest_derive::Parser;
use time::Date;
use zemen::Zemen;

use self::extractor::RawQuex;
use crate::error::QuexError;

type Result<'a, T> = std::result::Result<T, QuexError<'a>>;

#[enum_dispatch::enum_dispatch]
trait DisplayDate {
    fn date(&self) -> (i32, String, u8);
}

#[enum_dispatch::enum_dispatch(DisplayDate)]
#[derive(Debug, PartialEq)]
pub enum Calender {
    Date,
    Zemen,
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

    // for raw_quex in raw_quexs {
    let Some(schedule_list) = QuexParser::parse(Rule::schedule_list, raw_quex)?.next() else {
        return Ok(schedules);
    };

    for schedule in schedule_list.into_inner() {
        let mut schedule = schedule.into_inner();
        let Some(date) = schedule.next() else {
            continue;
        };
        let mut description = schedule.next().unwrap().as_str().to_string(); // won't fail

        match date.as_rule() {
            Rule::gregorian_date => {
                let mut gregorian_date = date.into_inner();
                let r = gregorian_date.as_str();

                let year = gregorian_date.next().unwrap(); // won't fail
                let mut year_str = year.as_str();

                if year.as_rule() == Rule::named_yearly {
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
                    year_str.parse().unwrap(),
                    month_from_quex(month.as_str()),
                    day.as_str().parse().unwrap(),
                )
                .map_err(|e| QuexError::GregorianDate(e, r))?;

                schedules.push(Schedule::new(description, Calender::from(schedule_date)));
            }
            Rule::recurring_monthly => {
                let today: time::Date = time::OffsetDateTime::now_utc().date();
                let raw_date = date;
                let date = raw_date
                    .as_str()
                    .strip_prefix("d=")
                    .and_then(|n| n.parse::<u8>().ok())
                    .unwrap();

                let mut month = today.month();

                if date < today.day() {
                    month = today.month().next();
                }

                let schedule_date = time::Date::from_calendar_date(today.year(), month, date)
                    .map_err(|e| QuexError::RecurringMonthly(e, raw_date.as_str()))?;

                schedules.push(Schedule::new(description, Calender::from(schedule_date)));
            }
            Rule::ethiopian_date => {
                let mut ethiopian_date = date.into_inner();
                let r = ethiopian_date.as_str();

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

                // this could still fail because we are not validating the range of the inputs
                let schedule_date = Zemen::from_eth_cal(
                    year_str.parse().unwrap(),
                    werh_from_quex(month.as_str()),
                    day.as_str().parse().unwrap(),
                )
                .map_err(|e| QuexError::EthiopianDate(e, r))?;

                schedules.push(Schedule::new(description, Calender::from(schedule_date)));
            }
            _ => unreachable!(),
        }
    }
    // }

    Ok(schedules)
}

fn werh_from_quex(as_str: &str) -> zemen::Werh {
    match as_str {
        "mes" | "መስከ" => zemen::Werh::Meskerem,
        "tik" | "ጥቅም" => zemen::Werh::Tikimit,
        "hed" | "ህዳር" => zemen::Werh::Hedar,
        "tah" | "ታኅሣ" => zemen::Werh::Tahasass,
        "tir" | "ታር" => zemen::Werh::Tir,
        "yek" | "የካቲ" => zemen::Werh::Yekatit,
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
