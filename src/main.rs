use clap::Parser;
use quex::{
    cli::{Cli, Command, Config},
    JulianDayNumber, Schedule, Schedules,
};

fn main() {
    let Cli {
        config,
        quex,
        editor,
        command,
        future,
        past,
        errors: _,
        format,
    } = Cli::parse();

    let app_config: Config = config
        .as_ref()
        .map(confy::load_path)
        .unwrap_or(confy::load("quex", "config"))
        .expect("can't open config file");

    let quex_path = quex.unwrap_or(app_config.calendar);
    let editor = editor.unwrap_or(std::env::var("EDITOR").unwrap_or(app_config.editor));

    // Commands
    if let Some(Command::Edit) = &command {
        quex::edit_schedules(quex_path.clone(), editor);
    }

    // Filtering options
    let (schedules, _parse_errors) = quex::get_schedules(quex_path.clone());

    let filter_options =
        filter_options(command.as_ref()).unwrap_or(FilterOptions::Ranged { future, past });

    let schedules = filter_schedules(schedules, filter_options);

    quex::view_schedules(schedules, &format);
}

fn filter_schedules(
    mut schedules: Schedules,
    filter_options: FilterOptions,
) -> Vec<(i32, Schedule)> {
    let jdn_today = time::OffsetDateTime::now_utc().date().to_julian_day();
    schedules.sort_by_key(|sch| sch.date.julian_day());

    match filter_options {
        FilterOptions::Ranged { future, past } => schedules
            .into_iter()
            .filter_map(|sch| {
                let diff = sch.date.julian_day() - jdn_today;

                match diff < future && diff > -past {
                    true => Some((diff, sch)),
                    false => None,
                }
            })
            .collect(),
        FilterOptions::All => schedules
            .into_iter()
            .filter_map(|sch| Some((sch.date.julian_day() - jdn_today, sch)))
            .collect(),
    }
}

fn filter_options(command: Option<&Command>) -> Option<FilterOptions> {
    match command {
        Some(c) => match c {
            Command::Week => Some(FilterOptions::Ranged { future: 7, past: 1 }),
            Command::Month => Some(FilterOptions::Ranged {
                future: 30,
                past: 1,
            }),
            Command::Year => Some(FilterOptions::Ranged {
                future: 365,
                past: 1,
            }),
            Command::All => Some(FilterOptions::All),
            _ => None,
        },
        None => None,
    }
}

enum FilterOptions {
    Ranged { future: i32, past: i32 },
    All,
}
