use std::{
    io::{stdin, stdout, Read, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueEnum};
use serde_derive::{Deserialize, Serialize};

use crate::filter::DateWindow;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Subcommands
    #[clap(subcommand)]
    pub command: Option<Command>,
    // -------------------------
    /// path to config file
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// path to calendar file
    #[clap(short, long)]
    pub quex: Option<PathBuf>,

    /// command to open calendar file
    #[clap(short, long)]
    pub editor: Option<String>,

    /// How many days into the future the report extends [default: 14]
    #[clap(short, long)]
    pub future: Option<i32>,

    /// How many days into the past the report extends [default: 3]
    #[clap(short, long)]
    pub past: Option<i32>,

    /// Show parsing errors
    #[clap(long)]
    pub errors: Option<bool>,

    /// Specify the format to use for printing the schedules [default: plain]
    #[clap(long, value_enum)]
    pub format: Option<Format>,

    /// Filter using a sub string
    #[clap(long)]
    pub filter: Option<String>,

    /// Filter by window of time
    #[clap(long)]
    pub date_window: Option<DateWindow>,
}

#[derive(Debug, PartialEq, ValueEnum, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Json,
    Plain,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(
        name = "edit",
        alias = "e",
        about = "edit calendar file",
        long_about = "open the calendar file with the configured editor, default is nvim"
    )]
    Edit,

    #[clap(name = "week", alias = "w", about = "view schedules file for the week")]
    Week,

    #[clap(
        name = "month",
        alias = "m",
        about = "view schedules file for the month"
    )]
    Month,

    #[clap(name = "year", alias = "y", about = "view schedules file for the year")]
    Year,

    #[clap(name = "all", alias = "a", about = "view schedules file for all time")]
    All,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub calendar: PathBuf,
    pub editor: String,
    pub future: Option<i32>,
    pub past: Option<i32>,
    pub print_errors: Option<bool>,
    pub format: Option<Format>,
}

impl Default for Config {
    fn default() -> Self {
        let calendar = confy::get_configuration_file_path("quex", "config")
            .expect("Can't get config path")
            .parent()
            .map(|p| p.join("calendar/"))
            .unwrap();

        Self {
            calendar,
            editor: String::from("nvim"),
            future: None,
            past: None,
            print_errors: None,
            format: None,
        }
    }
}

pub fn load_create_config(path: Option<impl AsRef<Path>>) -> Result<Config, String> {
    match path {
        Some(path) => {
            if !path.as_ref().exists() {
                println!("The config path: {:?}, doesn't exist", path.as_ref());
                print!("Would you like me to create it [y/N]: ");
                stdout().flush().expect("Something wrong with stdout");

                let confirmation = &mut [0; 1];
                stdin()
                    .read_exact(confirmation)
                    .expect("Can't get handle to stdin");

                if confirmation[0] != 121 {
                    std::process::exit(-1);
                }
            }

            confy::load_path(path.as_ref()).map_err(|_| format!("config file: {:?}", path.as_ref()))
        }
        None => {
            let config_path = confy::get_configuration_file_path("quex", "config")
                .expect("Error finding config directory");

            if !config_path.exists() {
                println!("The config path: {config_path:?}, doesn't exist");
                print!("Would you like me to create it [y/N]: ");
                stdout().flush().expect("Something wrong with stdout");

                let confirmation = &mut [0; 1];
                stdin()
                    .read_exact(confirmation)
                    .expect("Can't get handle to stdin");

                if confirmation[0] != 121 {
                    std::process::exit(-1);
                }
            }

            confy::load("quex", "config").map_err(|_| format!("config file: {:?}", config_path))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_file() {}
}
