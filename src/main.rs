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
        errors,
        format,
        filter: filter_str,
        date_window,
    } = Cli::parse();

    let app_config = cli::load_create_config(config).expect("Error loading config file");

    let quex_path = quex.unwrap_or(app_config.calendar);
    let editor = editor.unwrap_or(std::env::var("EDITOR").unwrap_or(app_config.editor));
    let errors = errors.unwrap_or(app_config.print_errors.unwrap_or(false));
    let format = format.unwrap_or(app_config.format.unwrap_or(quex::Format::Plain));

    let future = future.unwrap_or(app_config.future.unwrap_or(14));
    let past = past.unwrap_or(app_config.past.unwrap_or(3));

    // Commands
    if let Some(Command::Edit) = &command {
        quex::edit_schedules(quex_path.clone(), editor);
    }

    // Filtering options
    let (schedules, parse_errors) = quex::get_schedules(quex_path.clone());

    let date_winodw_filter = date_window.map(FilterOption::date_window);
    let range_filter = Some(FilterOption::new_ranged(future, past));
    let command_filter = filter::command_to_filter(command.as_ref()).or(range_filter);
    let sub_str_filter = filter_str.map(FilterOption::new_sub_str);

    let pipeline = vec![command_filter, sub_str_filter, date_winodw_filter];

    let schedules = filter::filter_pipeline(schedules, pipeline);

    // print the schedules
    quex::view_schedules(schedules, &format);
    if errors {
        quex::view_parse_errors(parse_errors, &format);
    }
}
