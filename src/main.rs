use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod scanner;
mod hasher;
mod reporter;

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

    run(cli.directories, cli.action);
}

fn run(directories: Vec<PathBuf>, action: Action) {
    let files = scanner::scan_directories(directories);
    let duplicates = reporter::find_duplicates(files);
    match action {
        Action::Report => {
            reporter::report_duplicates(duplicates);
        }
        Action::Move => {
            println!("Moving files");
        }
        Action::Delete => {
            println!("Deleting files");
        }
        Action::Symlink => {
            println!("Creating symlinks");
        }
    }
}
