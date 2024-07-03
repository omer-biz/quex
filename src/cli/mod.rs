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
