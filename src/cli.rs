use clap::{Parser, Subcommand};

/// A small tool to quickly log activity
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,

    /// Activate verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Register a new log entry
    Register,

    /// List registered entries
    List,
}
