use pest::Parser as _;
use pest_derive::Parser;
use zemen::Zemen;

use crate::calender::{ColumnLocation, DateInfo, DateResult, Event, LineError};

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

impl DateInfo for Zemen {
    fn julian_day(&self) -> i32 {
        self.to_jdn()
    }

    // TODO: allow the user to format the date
    fn pretty_print(&self) -> String {
        self.format("MMM D, YYYY")
    }
}

#[derive(Parser)]
#[grammar = "parser/grammar/base.pest"]
#[grammar = "parser/grammar/eth.pest"]
pub struct EthQuexParser;

pub fn parse_eth_quex(line: &str) -> DateResult<impl DateInfo> {
    let schedule = match EthQuexParser::parse(Rule::schedule, line) {
        Ok(s) => s,
        Err(e) => {
            // TODO: if you encounter any funny bugs, you should check this place.
            if e.variant.message() == "expected schedule" {
                // I can not parse this -> VError::NoParserFound
                return Ok(None);
            }
            // Found parsing error
            return Err(LineError::ParsingError {
                error: e.to_string(),
                message: e.variant.message().to_string(),
                column: ColumnLocation::new(e.location),
            });
        }
    };

    let today = Zemen::today();
    let mut schedule = schedule.into_iter().next().unwrap().into_inner();

    match schedule.peek().unwrap().as_rule() {
        Rule::ethiopian_date => {
            let mut date = schedule.next().unwrap().into_inner();
            let mut message = schedule.next().unwrap().as_str().to_string();

            let month = werh_from_quex(date.next().unwrap().as_str());
            let day: u8 = date.next().unwrap().as_str().parse().unwrap();

            let year = date.next().unwrap();

            let year = if year.as_rule() == Rule::named_yearly {
                let year = year.into_inner().next().unwrap();
                let yearn = year.as_str().parse::<i32>().unwrap();
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
                Zemen::today().year()
            } else {
                year.as_str().parse::<i32>().unwrap()
            };

            let date = match Zemen::from_eth_cal(year, month, day) {
                Ok(d) => d,
                // The user has submitted an invalid value
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

            // TODO: handle the case where the user inputs as date 30 in January
            // but as soon as the date passes the user will get an error because there is
            // no 30 in February
            if day < today.day() {
                month = month.next();
            }

            let date = match Zemen::from_eth_cal(today.year(), month, day) {
                Ok(d) => d,
                // additional context is need as per described by the TODO above.
                //
                // The user has submitted an invalid value
                Err(e) => return Err(LineError::InvalidValue(e.to_string())),
            };

            let message = schedule.next().unwrap().as_str().to_string();

            Ok(Some(Event::new(date, message)))
        }
        _ => unreachable!(),
    }
}
