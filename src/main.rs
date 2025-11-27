use std::path::Path;
use crate::cli::{parse_args, Commands};
use crate::scanner::scan;
use colored::Colorize;

mod scanner;
mod grouping;
mod cli;
mod mover;
mod ui;
mod config;

fn main() {
   // println!("Rust File Organizer — development in progress...");

    let args = parse_args();

    match args.command {
        Commands::Scan { path } => {
           check_if_file_exists(&path);

            println!("{} {}", "Scanning: ".cyan(), path);
            scan(&path);

        }
        Commands::Organize {path, dry_run} => {

            check_if_file_exists(&path);

            println!("{} {}", "Organize target:".cyan(), path);
            println!("{} {}", "Dry-run:".red(), dry_run);

            if dry_run {
                println!("{}", "Dry-run mode: no files will be moved.".red().bold());
            } else {
                // TODO: milestone v0.4
            }
        }
    }

}

fn check_if_file_exists<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if !path.exists() {
        eprintln!("{} {}", "Error: path does not exist: ".red().bold(), path.display());
        std::process::exit(1);
    }
}