use crate::config::Config;
use clap::Parser;
use log::info;
use log::LevelFilter::{Info, Warn};
use simple_logger::SimpleLogger;
use std::io::{self};
use organizer::Organizer;

mod organizer;
mod file_entry;
mod config;
mod utils;

#[derive(Parser)]
struct Args {
    /// The target directory to operate on.
    #[arg(short, long)]
    directory: String,

    /// Enable verbose output.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    /// Path to the configuration file.
    #[arg(short, long, default_value = "rules.toml")]
    config_path: String,

    /// Sort files.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    sort_files: bool,

    /// Move duplicates to a separate folder.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    move_duplicates: bool,

    /// Remove empty folders.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    remove_empty_folders: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    SimpleLogger::new()
        .with_level(if args.verbose { Info } else { Warn })
        .init()
        .unwrap();

    let config = Config::new(&args.directory)?;
    let mut organizer = Organizer::new(config)?;

    if args.sort_files {
        info!("Sorting files...");
        organizer.sort_all_files()?;
    }

    if args.move_duplicates {
        info!("Moving duplicates...");
        organizer.move_duplicates()?;
    }

    if args.remove_empty_folders {
        info!("Removing empty folders...");
        organizer.remove_empty_folders()?;
    }

    Ok(())
}