use std::path::PathBuf;

use crate::parser::JulianDayNumber;

pub mod cli;
mod error;
mod parser;

pub fn view_schedules(path: PathBuf, _json: Option<bool>) {
    let (schedules, _parse_errors) = parser::walker::walk_dir(&path).unwrap();
    let jdn_today = time::OffsetDateTime::now_utc().date().to_julian_day();

    schedules
        .iter()
        .filter(|sch| {
            let diff = sch.date.julian_day() - jdn_today;
            diff < 14 && diff > -3
        })
        .for_each(|sch| println!("{}", sch));
}
