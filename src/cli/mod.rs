use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde_derive::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_file() {}
}
