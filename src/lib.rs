use std::{path::PathBuf, process::Command};

pub use parser::walker::{QErrors, Schedules};

pub use crate::parser::JulianDayNumber;
pub use crate::parser::Schedule;

pub mod cli;
mod error;
mod parser;

pub fn get_schedules(path: PathBuf) -> (Schedules, QErrors) {
    parser::walker::walk_dir(&path).unwrap()
}

pub fn view_schedules(schedules: Vec<(i32, Schedule)>) {
    schedules.iter().for_each(|(diff, sch)| {
        println!(
            "{}{}",
            match diff {
                0 => "Today, ",
                1 => "Tomorrow, ",
                -1 => "Yesterday, ",
                _ => "",
            },
            sch
        )
    });
}

pub fn edit_schedules(path: PathBuf, editor: String) {
    Command::new(editor).arg(path).status().unwrap();
}
