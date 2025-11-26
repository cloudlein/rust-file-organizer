use std::path::Path;
use crate::cli::parse_args;
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
    
    if !Path::new(&args.path).exists() {
        eprintln!("{} {}", "Error: path does not exist: ".red().bold(), args.path);
        std::process::exit(1);
    }

    println!("{} {}", "Scanning: ".cyan(), args.path);
    println!("{} {}", "Dry-run: ".red(), args.dry_run);

    let groups = scan(&args.path);

    if args.dry_run {
        println!("{}", "Dry-run mode: no files will be moved.".red().bold());
    }else {
        // TODO: will implemented after milestone v0.4
    }

}
