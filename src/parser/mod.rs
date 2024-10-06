pub mod date_window;
pub mod schedule;
pub mod walker;

#[cfg(feature = "eth")]
use crate::calender::eth::parse_eth_quex;

use crate::calender::gre::parse_gre_quex;
use crate::calender::LineError;
use crate::Schedule;

// The length I go to mimic parser combinators ;)
macro_rules! run_parsers {
    () => {};

    ($line:expr, $first:expr, $($func:expr),*) => {
        match $first($line) {
            Ok(Some(event)) => return Ok(event.into()),
            Ok(None) => { run_parsers!($line, $($func),*)  }
            Err(e) => return Err(e),
        }
    };

    ($line:expr, $last:expr) => {
        match $last($line){
            Ok(Some(event)) => return Ok(event.into()),
            Ok(None) => return Err(LineError::CantParseInput), // I have run out of parsers to try
            Err(e) => return Err(e)
        }
    };

}

pub fn parse_line(line: &str) -> Result<Schedule, LineError> {
    #[cfg(feature = "eth")]
    run_parsers!(line, parse_gre_quex, parse_eth_quex);
    #[cfg(not(feature = "eth"))]
    run_parsers!(line, parse_gre_quex);
}

#[cfg(test)]
mod tests {
    //     use super::Calender;
    //     use super::Schedule;
    //     use time::Date;

    //     #[cfg(feature = "eth")]
    //     use zemen::Zemen;

    //     #[test]
    //     fn test_parse_quex() {
    //         let input = r#"2024 mar 1, sample description.
    // d=5, recurring monthly
    // 1992* feb 29, reacurring yeal: year: \y and past_time: \a
    // "#;

    //         let today = time::OffsetDateTime::now_utc();

    //         let month = if today.day() > 5 {
    //             today.month().next()
    //         } else {
    //             today.month()
    //         };

    //         let output = vec![
    //             Schedule {
    //                 description: "sample description.".to_string(),
    //                 date: Calender::from(
    //                     Date::from_calendar_date(2024, time::Month::March, 1).unwrap(),
    //                 ),
    //                 time: None,
    //                 diff: None,
    //             },
    //             Schedule {
    //                 description: "recurring monthly".to_string(),
    //                 date: Calender::from(Date::from_calendar_date(today.year(), month, 5).unwrap()),
    //                 time: None,
    //                 diff: None,
    //             },
    //             Schedule {
    //                 description: "reacurring yeal: year: 1992 and past_time: 32".to_string(),
    //                 date: Calender::from(
    //                     Date::from_calendar_date(today.year(), time::Month::February, 29).unwrap(),
    //                 ),
    //                 time: None,
    //                 diff: None,
    //             },
    //         ];

    //         let schedules = super::parse_quex(input).unwrap();
    //         assert_eq!(schedules, output);
    //     }

    //     #[test]
    //     #[cfg(feature = "eth")]
    //     fn test_parse_eth() {
    //         let input = r#"2016 neh 1, in ethiopia
    // 2015* mes 1, another eth  this \y and this \a
    // "#;

    //         let today = Zemen::today();

    //         let output = vec![
    //             super::Schedule {
    //                 description: "in ethiopia".to_string(),
    //                 date: Calender::from(Zemen::from_eth_cal(2016, zemen::Werh::Nehase, 1).unwrap()),
    //                 time: None,
    //                 diff: None,
    //             },
    //             super::Schedule {
    //                 description: "another eth  this 2015 and this 2".to_string(),
    //                 date: Calender::from(
    //                     Zemen::from_eth_cal(today.year(), zemen::Werh::Meskerem, 1).unwrap(),
    //                 ),
    //                 time: None,
    //                 diff: None,
    //             },
    //         ];

    //         let schedules = super::parse_quex(input).unwrap();
    //         assert_eq!(schedules, output);
    //     }
}
