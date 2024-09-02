use std::{path::PathBuf, process::Command};

pub use crate::parser::schedule::JulianDayNumber;
pub use crate::parser::schedule::Schedule;
pub use cli::Format;
pub use parser::walker::{QErrors, Schedules};

pub mod cli;
mod error;
mod parser;

pub fn get_schedules(path: PathBuf) -> (Schedules, QErrors) {
    parser::walker::walk_dir(&path).unwrap()
}

pub fn view_schedules(schedules: Vec<(i32, Schedule)>, format: &Format) {
    match format {
        Format::Json => {
            let json = serde_json::to_string(&schedules).unwrap();
            println!("{}", json);
        }
        Format::Plain => schedules.iter().for_each(|(diff, sch)| {
            let time = sch
                .time
                .as_ref()
                .map(|t| format!(", {t}"))
                // .map(|t| t.to_string())
                .unwrap_or("".to_string());

            match diff {
                0 => println!("Today{}, {}", time, sch.description),
                1 => println!("Tomorrow{}, {}", time, sch.description),
                -1 => println!("Yesterday{}, {}", time, sch.description),
                _ => println!("{}", sch),
            }
        }),
    }
}

pub fn view_parse_errors(errors: QErrors, format: &Format) {
    match format {
        Format::Json => {
            let json = serde_json::to_string(&errors).unwrap();
            println!("{}", json);
        }
        Format::Plain => errors.iter().for_each(|err| print!("{}", err)),
    }
}

pub fn edit_schedules(path: PathBuf, editor: String) {
    Command::new(editor).arg(path).status().unwrap();
}
