use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod deleter;
mod hasher;
mod reporter;
mod scanner;
mod symlinker;

#[derive(Debug, Clone, ValueEnum)]
enum Action {
    Report,
    Delete,
    Symlink,
}

// Define the command-line arguments struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of directories
    #[arg(required = true)]
    directories: Vec<PathBuf>,

    /// Action to perform
    #[arg(short, long, value_enum)]
    action: Action,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli.directories, cli.action) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(directories: Vec<PathBuf>, action: Action) -> Result<(), Box<dyn std::error::Error>> {
    let files = scanner::scan_directories(directories)?;
    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    let duplicates = reporter::find_duplicates(files)?;
    if duplicates.is_empty() {
        println!("No duplicates found.");
        return Ok(());
    }

    match action {
        Action::Report => Ok(reporter::report_duplicates(duplicates)),
        Action::Delete => {
            match deleter::delete(duplicates){
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        },
        Action::Symlink => {
            match symlinker::symlink_files(&duplicates) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}
