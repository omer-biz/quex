use std::{collections::HashMap, error::Error};

use clap::Parser;
use quex::{
    cli::{self, Cli, Command},
    filter::{self, FilterOption},
};

fn main() {
    let Cli {
        config,
        quex,
        editor,
        command,
        future,
        past,
        format,
        filter: filter_str,
        date_window,
        file_format,
        block,
    } = Cli::parse();

    if file_format.len() != block.len() {
        eprintln!("Error: The number of file formats and blocks must match.");
        return;
    }

    let file_format: Result<HashMap<String, String>, Box<dyn Error>> = file_format
        .into_iter()
        .zip(block.into_iter())
        .map(|(ff, bb)| {
            let bparts: Vec<&str> = bb.split(',').collect();

            if bparts.len() != 2 {
                return Err(format!(
                    "Block '{}' must contain a `start` and `end` separated by a comma.",
                    bb
                )
                .into());
            }

            Ok((ff, bb))
        })
        .collect();

    let file_format = file_format.unwrap();

    let app_config = cli::load_create_config(config).expect("Error loading config file");

    let quex_path = quex.unwrap_or(app_config.calendar);
    let editor = editor.unwrap_or(std::env::var("EDITOR").unwrap_or(app_config.editor));
    let format = format.unwrap_or(app_config.format.unwrap_or(quex::Format::Plain));

    let future = future.unwrap_or(app_config.future.unwrap_or(14));
    let past = past.unwrap_or(app_config.past.unwrap_or(3));

    // Commands
    if let Some(Command::Edit) = &command {
        quex::edit_schedules(quex_path.as_path(), editor);
    }

    // Filtering options
    let (schedules, parse_errors) = quex::get_schedules(quex_path, file_format);

    let date_window_filter = date_window.map(FilterOption::date_window);
    let range_filter = Some(FilterOption::new_ranged(future, past));
    let command_filter = filter::command_to_filter(command.as_ref()).or(range_filter);
    let sub_str_filter = filter_str.map(FilterOption::new_sub_str);

    let pipeline = vec![command_filter, sub_str_filter, date_window_filter];

    let schedules = filter::filter_pipeline(schedules, pipeline);

    // print the schedules
    quex::view_schedules(schedules, &format);
    quex::view_parse_errors(parse_errors, &format);
}
