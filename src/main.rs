use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod deleter;
mod hasher;
mod reporter;
mod scanner;

#[derive(Debug, Clone, ValueEnum)]
enum Action {
    Report,
    Move,
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
    let duplicates = reporter::find_duplicates(files)?;
    match action {
        Action::Report => Ok(reporter::report_duplicates(duplicates)),

        Action::Move => {
            println!("Moving files");
            Ok(())
        }
        Action::Delete => Ok(deleter::delete(duplicates)),
        Action::Symlink => {
            println!("Creating symlinks");
            Ok(())
        }
    }
}
