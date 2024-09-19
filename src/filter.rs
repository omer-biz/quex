pub(crate) use crate::{cli::Command, parser::date_window::DateWindow, Schedules};

pub enum FilterOption {
    Ranged { future: i32, past: i32 },
    All,
    SubStr(String),
    DateWinodw(DateWindow),
}

impl FilterOption {
    pub fn new_ranged(future: i32, past: i32) -> Self {
        Self::Ranged { future, past }
    }

    pub fn new_sub_str(sub_str: String) -> Self {
        Self::SubStr(sub_str)
    }
    pub fn date_window(dw: DateWindow) -> Self {
        Self::DateWinodw(dw)
    }
}

fn filter_schedules(mut schedules: Schedules, filter_options: Option<FilterOption>) -> Schedules {
    if let Some(filter_options) = filter_options {
        let jdn_today = time::OffsetDateTime::now_utc().date().to_julian_day();
        schedules.sort_by_key(|sch| sch.date.julian_day());

        match filter_options {
            FilterOption::Ranged { future, past } => schedules
                .into_iter()
                .filter_map(|mut sch| {
                    let diff = sch.date.julian_day() - jdn_today;
                    sch.diff = Some(diff);

                    match diff < future && diff > -past {
                        true => Some(sch),
                        false => None,
                    }
                })
                .collect(),
            FilterOption::All => schedules
                .into_iter()
                .map(|mut sch| {
                    let diff = sch.date.julian_day() - jdn_today;
                    sch.diff = Some(diff);
                    sch
                })
                .collect(),
            FilterOption::SubStr(sub_str) => schedules
                .into_iter()
                .filter(|sch| sch.description.contains(sub_str.as_str()))
                .collect(),

            FilterOption::DateWinodw(DateWindow { begin, end }) => {
                let end = end.unwrap_or(jdn_today);

                schedules
                    .into_iter()
                    .filter(|sch| sch.date.julian_day() >= begin && sch.date.julian_day() <= end)
                    .collect()
            }
        }
    } else {
        schedules
    }
}

pub fn command_to_filter(command: Option<&Command>) -> Option<FilterOption> {
    match command {
        Some(c) => match c {
            Command::Week => Some(FilterOption::Ranged { future: 7, past: 1 }),
            Command::Month => Some(FilterOption::Ranged {
                future: 30,
                past: 1,
            }),
            Command::Year => Some(FilterOption::Ranged {
                future: 365,
                past: 1,
            }),
            Command::All => Some(FilterOption::All),
            _ => None,
        },
        None => None,
    }
}

pub fn filter_pipeline(schedules: Schedules, pipeline: Vec<Option<FilterOption>>) -> Schedules {
    pipeline.into_iter().fold(schedules, filter_schedules)
}
