use rustyline::DefaultEditor;
use std::path::PathBuf;

pub fn get_player_input(history_path: PathBuf, verbose: bool) -> String {
    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history(&history_path).is_err() && verbose {
        println!("-> Creating new history at {}", history_path.display());
    }

    match rl.readline("Enter your entry: ") {
        Ok(line) => {
            let trimmed = line.trim().to_string();
            if !trimmed.is_empty() {
                rl.add_history_entry(trimmed.as_str()).unwrap();
            }
            if let Err(e) = rl.save_history(&history_path) {
                eprintln!("failed to save history: {e}");
            }

            trimmed
        }
        Err(e) => {
            eprintln!("failed to read input: {e}");
            String::new()
        }
    }
}
