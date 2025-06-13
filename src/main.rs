mod cli;
mod core;
use core::{get_cache_dir, get_player_input, list_entries, save_entry};

use chrono::Local;
use clap::Parser;
use cli::Action;

fn main() {
    let args = cli::Cli::parse();

    let mut history_path = get_cache_dir(args.verbose);
    history_path.push("history");

    dbg!(&history_path);

    match args.action {
        Action::Register => {
            save_entry(
                format_entry(get_player_input(history_path, args.verbose)),
                args.verbose,
            );
        }

        Action::List => {
            list_entries(args.verbose);
        }
    }
}

fn format_entry(entry: String) -> String {
    let now = Local::now();
    format!("{} {}", now.format("%Y-%m-%d %H:%M:%S"), entry)
}
