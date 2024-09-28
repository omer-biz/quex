pub(crate) use crate::{cli::Command, parser::date_window::DateWindow, Schedules};

pub enum FilterOption {
    Ranged { future: i32, past: i32 },
    All,
    SubStr(String),
    DateWindow(DateWindow),
}

impl FilterOption {
    pub fn new_ranged(future: i32, past: i32) -> Self {
        Self::Ranged { future, past }
    }

    pub fn new_sub_str(sub_str: String) -> Self {
        Self::SubStr(sub_str)
    }
    pub fn date_window(dw: DateWindow) -> Self {
        Self::DateWindow(dw)
    }
}

fn filter_schedules(mut schedules: Schedules, filter_options: Option<FilterOption>) -> Schedules {
    if let Some(filter_options) = filter_options {
        schedules.sort_by_key(|sch| sch.julian_day_number);

        match filter_options {
            FilterOption::Ranged { future, past } => schedules
                .into_iter()
                .filter_map(|sch| match sch.diff < future && sch.diff > -past {
                    true => Some(sch),
                    false => None,
                })
                .collect(),
            FilterOption::All => schedules,
            FilterOption::SubStr(sub_str) => schedules
                .into_iter()
                .filter(|sch| sch.description.contains(sub_str.as_str()))
                .collect(),

            FilterOption::DateWindow(DateWindow { begin, end }) => schedules
                .into_iter()
                .filter(|sch| sch.julian_day_number >= begin && sch.julian_day_number <= end)
                .collect(),
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
