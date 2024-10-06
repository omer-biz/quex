use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub use crate::parser::schedule::Schedule;
pub use cli::Format;
pub use parser::walker::{QErrors, Schedules};

pub mod cli;
mod error;
pub mod filter;
mod parser;

pub mod calender;

pub fn get_schedules(path: PathBuf) -> (Schedules, QErrors) {
    parser::walker::walk_dir(path).unwrap()
}

pub fn view_schedules(schedules: Schedules, format: &Format) {
    match format {
        Format::Json => {
            let json = serde_json::to_string(&schedules).unwrap();
            println!("{}", json);
        }
        Format::Plain => schedules.iter().for_each(|sch| {

            match sch.diff {
                0 => println!("Today; {}", sch.description),
                1 => println!("Tomorrow; {}", sch.description),
                -1 => println!("Yesterday; {}", sch.description),
                _ => println!("{}; {}", sch.date, sch.description),
            }
        }),
    }
}

pub fn view_parse_errors(errors: QErrors, format: &Format) {
    match format {
        Format::Json => {
            let json = serde_json::to_string(&errors).unwrap();
            eprintln!("{}", json);
        }
        Format::Plain => errors
            .into_iter()
            .for_each(|err| eprint!("{}", err.format())),
    }
}

pub fn edit_schedules(path: &Path, editor: String) {
    Command::new(editor).arg(path).status().unwrap();
}
