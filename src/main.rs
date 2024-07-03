use clap::Parser;
use quex::cli::{Cli, Command, Config};

fn main() {
    let Cli {
        config,
        quex,
        editor,
        command,
    } = Cli::parse();

    let app_config: Config = config
        .as_ref()
        .map(|path| confy::load_path(path))
        .unwrap_or(confy::load("quex", "config"))
        .expect("can't open config file");

    let quex_path = quex.unwrap_or(app_config.calendar);
    let _editor = editor.unwrap_or(app_config.editor);

    match command {
        Some(Command::Edit) => todo!(),
        None => quex::view_schedules(quex_path, None),
    }
}
