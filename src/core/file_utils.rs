use dirs::cache_dir;
use std::fs::{OpenOptions, create_dir_all};
use std::io::prelude::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn get_cache_dir(verbose: bool) -> PathBuf {
    let mut path = cache_dir().expect("could not determine cache directory");
    path.push(PKG_NAME);
    create_dir_all(&path).expect("failed to create cache directory");
    if verbose {
        println!("-> Created cache directory at {}", path.display());
    }
    path
}

pub fn save_entry(entry: String, verbose: bool) {
    let mut path = get_cache_dir(verbose);
    path.push("entries");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .unwrap();

    if let Err(e) = writeln!(file, "{entry}") {
        eprintln!("could not write to the file: {e}");
    }

    if verbose {
        println!("-> Wrote entry to {}", &path.display());
    }
}

pub fn list_entries(verbose: bool) {
    let mut path = get_cache_dir(verbose);
    path.push("entries");

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(_) => return,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        println!("{line}");
    }
}
