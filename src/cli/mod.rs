use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Cli {
    /// path to config file
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// path to calendar file
    #[clap(short, long)]
    pub quex: Option<PathBuf>,

    /// command to open calendar file
    #[clap(short, long)]
    pub editor: Option<String>,

    /// Subcommands
    #[clap(subcommand)]
    pub command: Option<Command>,

    /// How many days into the future the report extends. Default: 14
    #[clap(short, long, default_value = "14")]
    pub future: i32,

    /// How many days into the past the report extends. Default: 3
    #[clap(short, long, default_value = "3")]
    pub past: i32,

    /// Show parsing errors
    #[clap(long, default_value = "false")]
    pub errors: bool,
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

    #[clap(name = "week", alias = "w", about = "view calendar file for the week")]
    Week,

    #[clap(
        name = "month",
        alias = "m",
        about = "view calendar file for the month"
    )]
    Month,

    #[clap(name = "year", alias = "y", about = "view calendar file for the year")]
    Year,

    #[clap(name = "all", alias = "a", about = "view calendar file for all time")]
    All,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub calendar: PathBuf,
    pub editor: String,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use crate::cli::Config;

    #[test]
    fn test_config_file() {
        let now = SystemTime::now();
        let a = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

        println!("time: {}", now.elapsed().unwrap().as_secs());
        println!("time: {:?}", now);
        println!("time: {}", a.as_secs());

        // let config = super::Config::default();
        // let config: Config = confy::load("quex", "config").unwrap();
        // confy::store("quex", "config", &config).unwrap();
        //
        // let a = confy::get_configuration_file_path("quex", "config").unwrap();
        // println!("path: {}", a.display());
    }
}
